import Vue from 'vue'
import Vuex from 'vuex'
import {cloneDeep} from 'lodash'
import * as api from './api'
import router from '../router';

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    accountForm: {
      username: null,
      password: null
    },
    editedTask: {
      id: null,
      title: null,
      completed: null,
      priority: null,
      description: null
    },
    editingOneTask: false,
    errorMessage: '',
    tasks: [],
    user: {
      username: null,
      token: null,
      id: null,
    }
  },
  mutations: {
    resetAccountForm(state) {
      Vue.set(state, 'accountForm', {username: null, password: null});
    },
    resetTasks(state, tasks = []) {
      Vue.set(state, 'tasks', tasks);
    },
    setAccountFormPassword(state, password) {
      Vue.set(state.accountForm, 'password', password);
    },
    setAccountFormUsername(state, username) {
      Vue.set(state.accountForm, 'username', username);
    },
    setEditedTask(state, editedTask) {
      Vue.set(state, "editedTask", editedTask);
    },
    setEditTaskDescription(state, taskDescription) {
      Vue.set(state.editedTask, "description", taskDescription);
    },
    setEditTaskTitle(state, taskTitle) {
      Vue.set(state.editedTask, "title", taskTitle);
    },
    setErrorMessage(state, errorMessage) {
      Vue.set(state, 'errorMessage', errorMessage);
    },
    setUser(state, user) {
      Vue.set(state, 'user', user);
    },
    turnOffEditTaskMode(state) {
      Vue.set(state, "editingOneTask", false);
    },
    turnOnEditTaskMode(state) {
      Vue.set(state, "editingOneTask", true);
    },
    updateTask(state, updatedTask) {
      const clonedTasks = cloneDeep(state.tasks);
      const taskIndex = clonedTasks.findIndex(task => task.id == updatedTask.id);
      if(!taskIndex) throw new Error(`could not find task with id ${updatedTask.id}`);
      clonedTasks[taskIndex] = updatedTask;
      Vue.set(state, "tasks", clonedTasks);
    }
  },
  actions: {
    async createAccount({commit, getters, state, dispatch}) {
      if(!getters.accountFormValid) {
        commit("setErrorMessage", "Can't create account, missing usersname and/or password");
        return;
      }

      try {
        const createdAccount = await api.createAccount(state.accountForm);
        commit('setUser', createdAccount.data);
        commit("resetAccountForm")
        dispatch("loadTasksFromApi");
        router.push("/")
      } catch (error) {
        commit("setErrorMessage", error.message);
      }
    },

    async loadTasksFromApi({state, commit}) {
      const tasks = await api.getTasks(state.user.token);
      commit("resetTasks", tasks);
    },

    async login({state, commit, getters, dispatch}) {
      if(!getters.accountFormValid) {
        commit("setErrorMessage", "Can't login, missing usersname and/or password");
        return;
      }

      try {
        const account = await api.login(state.accountForm);
        commit('setUser', account.data);
        commit("resetAccountForm");
        dispatch("loadTasksFromApi");
        router.push("/")
      } catch (error) {
        commit("setErrorMessage", error.message);
      }
    },
    async saveTask({state, commit}) {
      await api.updateTask(state.editedTask, state.user.token);
      commit("updateTask", state.editedTask);
      commit("turnOffEditTaskMode");
    },
    switchToEditMode({commit, state}, taskId) {
      if(!taskId) return commit("setErrorMessage", "No task ID, please try logging in and then retry");
      const currentTask = state.tasks.find(task => task.id == taskId);
      if(!currentTask) return commit("setErrorMessage", `Could not find task with id ${taskId}`);

      commit("setEditedTask", Object.assign({}, currentTask));
      commit("turnOnEditTaskMode");
    }
  },
  modules: {
  },
  getters: {
    accountFormValid(state) {
      return !!state.accountForm.username && !!state.accountForm.password
    },
    loggedIn(state) {
      return !!state.user.token
    },
    username(state) {
      return state.user.username
    }
  }
})

import fs from 'fs';
import { UserData } from './types/user_data';


const getUserData = (chatId: number): UserData => {
    const data = fs.readFileSync(`../user_data/${chatId}.json`, 'utf-8');
    return JSON.parse(data) as UserData;
}

const createUserData = (chatId: number): void => {
    fs.writeFileSync(`../user_data/${chatId}.json`, JSON.stringify({ topics: [] }));
}

const setUserData = (chatId: number, userData: UserData): void => {
    fs.writeFileSync(`../user_data/${chatId}.json`, JSON.stringify(userData));
}

const deleteUserData = (chatId: number): void => {
    try {
        fs.unlinkSync(`../user_data/${chatId}.json`);
    } catch (err) {
        console.error(err);
    }
}

const addUserTopics = (chatId: number, topics: string[]): void => {
    const userData: UserData = getUserData(chatId);
    const combinedTopics = userData.topics.concat(topics);
    userData.topics = combinedTopics;
    setUserData(chatId, userData);
}

export const userService = {
    getUserData,
    createUserData,
    setUserData,
    deleteUserData,
    addUserTopics
}


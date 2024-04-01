/* eslint-disable no-unused-expressions */
/* eslint-disable no-restricted-globals */
/* eslint-disable no-unused-vars */
/* eslint-disable eqeqeq */
import React, { useState, useCallback } from "react"
export const useLogin = (initialState = false) => {
    // Initialize the state
    const [state, setState] = useState(initialState);
    
    // Define and memorize toggler function in case we pass down the component,
    // This function change the boolean value to it's opposite value
    const login = useCallback(() => {
        setState(state => !state)
    }, []);
    
    return [state, login]
}

export const useToast =  {
    status:false,
    get is_open(){
        return this.status
    },
    set is_open(v){
        this.status = v
    }
}

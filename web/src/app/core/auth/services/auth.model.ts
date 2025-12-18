export type User = {
	id: string;
};

export type UserResponse = {
	data: {
		user_id: string;
	};
};

export type TokenResponse = {
	access_token: string;
	user_id: string;
};

export type TokenResult = {
	accessToken: string;
	userId: string;
};

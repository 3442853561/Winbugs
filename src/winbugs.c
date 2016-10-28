//winbugs.c
#include <stdlib.h>
#include <windows.h>

void wbMsgbox(char* text,char* title,int style)
{
	MessageBox (NULL,text,title,style);
}

int screenWidth()
{
	return GetSystemMetrics(SM_CXSCREEN);
}

int screenHeight()
{
	return GetSystemMetrics(SM_CYSCREEN);
}
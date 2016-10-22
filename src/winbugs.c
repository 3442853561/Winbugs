//winbugs.c
#include <windows.h>
void wbMsgbox(char* text,char* title)
{
	MessageBox (NULL,TEXT(text),TEXT(title),0);
}
int screenWidth()
{
	return GetSystemMetrics(SM_CXSCREEN);
}
int screenHeight()
{
	return GetSystemMetrics(SM_CYSCREEN);
}
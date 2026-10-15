
/* lpfn parameter of SetWindowsHookExA
    */

void lpfn_00442b90(int param_1,WPARAM param_2,int param_3)

{
  int iVar1;
  CHAR aCStack_100 [256];
  
  if ((-1 < param_1) && (param_1 != 3)) {
    iVar1 = GetClassNameA(*(HWND *)(param_3 + 8),aCStack_100,0xff);
    if (iVar1 != 0) {
      iVar1 = _strncmp(aCStack_100,s_Game_Menu_Window_006a8718,0xff);
      if ((iVar1 != 0) && ((param_2 == 0x201 || (param_2 == 0x204)))) {
        FUN_00442430();
      }
    }
  }
  CallNextHookEx(DAT_006b28f8,param_1,param_2,param_3);
  return;
}


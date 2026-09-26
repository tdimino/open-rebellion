
BOOL __fastcall FUN_004162a0(int param_1)

{
  HWND hWndMain;
  LPBYTE pBVar1;
  void *pvVar2;
  LPCSTR lpszHelp;
  BOOL BVar3;
  UINT uCommand;
  char *pcVar4;
  ULONG_PTR dwData;
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062acc8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(&local_70,1);
  local_4 = 0;
  pBVar1 = FUN_00401440();
  if (pBVar1 != (LPBYTE)0x0) {
    pcVar4 = &DAT_006a7c4c;
    pvVar2 = FUN_00616110(local_64,(char *)pBVar1);
    FUN_00616110(pvVar2,pcVar4);
  }
  pvVar2 = FUN_00616110(local_64,s_swhelp_hlp_006a8440);
  FUN_00615f00(pvVar2,0);
  hWndMain = *(HWND *)(param_1 + 0x18);
  dwData = 0;
  uCommand = 0xb;
  lpszHelp = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
  BVar3 = WinHelpA(hWndMain,lpszHelp,uCommand,dwData);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return BVar3;
}


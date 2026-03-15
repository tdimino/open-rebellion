
LPBYTE FUN_00401440(void)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  LPCSTR pCVar4;
  LPBYTE pBVar5;
  char *pcVar6;
  char *pcVar7;
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 local_5c [8];
  int iStack_3c;
  _RTL_CRITICAL_SECTION _Stack_38;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006290d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(&local_70,1);
  pcVar7 = s_Star_Wars_Rebellion_006a7c50;
  pcVar6 = &DAT_006a7c4c;
  local_4 = 0;
  pvVar3 = FUN_00616110(local_64,s_SOFTWARE_LucasArts_Entertainment_006a7c1c);
  pvVar3 = FUN_00616110(pvVar3,pcVar6);
  pvVar3 = FUN_00616110(pvVar3,pcVar7);
  FUN_00615f00(pvVar3,0);
  pcVar6 = s_DllLocation_006a7c10;
  pCVar4 = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
  pBVar5 = FUN_005f3fc0(pCVar4,pcVar6);
  FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(local_70 + 4)),0);
  iVar1 = *(int *)(local_70 + 4);
  if (*(int *)((int)&iStack_3c + iVar1) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar1));
  }
  iVar2 = *(int *)((int)&iStack_3c + iVar1);
  *(undefined4 *)((int)aiStack_6c + iVar1 + 4) = 0;
  if (iVar2 < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar1));
  }
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return pBVar5;
}


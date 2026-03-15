
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall FUN_00614050(void *this,char *param_1,int param_2)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  LPCSTR lpLibFileName;
  char *pcVar4;
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 auStack_5c [8];
  int iStack_3c;
  _RTL_CRITICAL_SECTION _Stack_38;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00657058;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)this = &PTR_FUN_0066e400;
  lpRect_006be6b8 = (LPRECT)0x0;
  _DAT_006be6bc = 0;
  _DAT_006be6c0 = 0;
  DAT_006be6d4 = 0;
  DAT_006be6dc = 0;
  DAT_006be6e0 = 0;
  DAT_006be6e4 = 0;
  DAT_006be6ec = 0;
  DAT_006be6e8 = 0;
  _DAT_006be6f0 = 0;
  _DAT_006be6f4 = 0;
  _DAT_006be6f8 = 0;
  DAT_006be6fc = 0;
  DAT_006be700 = 0;
  _DAT_006be6c4 = 0;
  FUN_00615db0(&local_70,1);
  local_4 = 0;
  if (param_1 != (char *)0x0) {
    pcVar4 = &DAT_006a7c4c;
    pvVar3 = FUN_00616110(local_64,param_1);
    FUN_00616110(pvVar3,pcVar4);
  }
  pvVar3 = FUN_00616110(local_64,PTR_s_cdderr_dll_006ac95c);
  FUN_00615f00(pvVar3,0);
  lpLibFileName = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
  DAT_006be704 = LoadLibraryA(lpLibFileName);
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
  if ((param_1 != (char *)0x0) && (param_2 != 0)) {
    FUN_00618b60(param_1);
  }
  local_4 = 0xffffffff;
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return this;
}


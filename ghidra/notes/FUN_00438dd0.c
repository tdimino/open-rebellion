
void __thiscall FUN_00438dd0(void *this,int param_1,int param_2,int param_3)

{
  HWND pHVar1;
  char *pcVar2;
  int iVar3;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d770;
  local_c = ExceptionList;
  pHVar1 = *(HWND *)((int)this + 0x18);
  iVar3 = param_2;
  if (9999 < param_2) {
    iVar3 = 9999;
  }
  if (9999 < param_3) {
    param_3 = 9999;
  }
  ExceptionList = &local_c;
  if (*(int *)((int)this + 0x134) != 0) {
    ExceptionList = &local_c;
    FUN_005f2f50(local_18);
    local_4 = 0;
    if (param_1 == 0) {
      param_2 = CONCAT22(DAT_0065d424,0x3816);
      FUN_005f3010(local_18,&param_2);
    }
    else {
      FUN_005f31a0(local_18,iVar3);
    }
    pcVar2 = (char *)FUN_00583c40((int)local_18);
    FUN_00601aa0(*(void **)((int)this + 0x134),pcVar2);
    FUN_00601dc0(*(void **)((int)this + 0x134),pHVar1,1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
  }
  if (*(int *)((int)this + 0x138) != 0) {
    FUN_005f2f50(local_18);
    local_4 = 1;
    if (param_1 == 0) {
      param_3 = CONCAT22(DAT_0065d424,0x3816);
      FUN_005f3010(local_18,&param_3);
    }
    else {
      FUN_005f31a0(local_18,param_3);
    }
    pcVar2 = (char *)FUN_00583c40((int)local_18);
    FUN_00601aa0(*(void **)((int)this + 0x138),pcVar2);
    FUN_00601dc0(*(void **)((int)this + 0x138),pHVar1,1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
  }
  ExceptionList = local_c;
  return;
}


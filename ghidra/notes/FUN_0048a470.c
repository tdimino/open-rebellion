
void __thiscall FUN_0048a470(void *this,int param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634ebf;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x50);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_00420f10(puVar1);
  }
  local_4 = 0xffffffff;
  FUN_005f3090(piVar2 + 0x11,param_1);
                    /* WARNING: Load size is inaccurate */
  piVar2[8] = *this;
  FUN_0041ce20(piVar2,0);
  ExceptionList = local_c;
  return;
}



bool __cdecl FUN_0053f870(int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  int *piVar3;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006469e0;
  local_c = ExceptionList;
  piVar3 = (int *)0x0;
  if (param_1 == (int *)0x0) {
    return false;
  }
  uVar2 = (uint)param_1[9] >> 4 & 3;
  if (param_1[6] == 3) {
    local_14[0] = 0xf1;
    local_14[1] = 0xf2;
    local_4 = 0;
    ExceptionList = &local_c;
    piVar3 = FUN_0053f030(local_14,uVar2,1);
  }
  else {
    ExceptionList = &local_c;
    bVar1 = FUN_005406d0(param_1);
    if (CONCAT31(extraout_var,bVar1) == 0) goto LAB_0053f92e;
    local_14[0] = 0xf2;
    local_14[1] = 0xf3;
    local_4 = 1;
    piVar3 = FUN_0053f030(local_14,uVar2,1);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_0053f92e:
  if (piVar3 != (int *)0x0) {
    FUN_0053a5d0(param_1);
  }
  ExceptionList = local_c;
  return param_1 != (int *)0x0;
}


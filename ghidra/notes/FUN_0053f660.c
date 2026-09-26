
bool __cdecl FUN_0053f660(int *param_1)

{
  int *this;
  bool bVar1;
  undefined3 extraout_var;
  int *this_00;
  bool bVar2;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006469a0;
  local_c = ExceptionList;
  bVar2 = param_1 != (int *)0x0;
  if (!bVar2) {
    return false;
  }
  ExceptionList = &local_c;
  FUN_005406b0(param_1);
  FUN_00540650(this,0);
  FUN_00540630(this,1);
  if (!bVar2) {
    ExceptionList = local_c;
    return false;
  }
  this_00 = (int *)0x0;
  if (this[6] == 3) {
    local_14[0] = 0xf1;
    local_14[1] = 0xf2;
    local_4 = 0;
    this_00 = FUN_0053f030(local_14,0,1);
  }
  else {
    bVar1 = FUN_005406d0(this);
    if (CONCAT31(extraout_var,bVar1) == 0) goto LAB_0053f734;
    local_14[0] = 0xf2;
    local_14[1] = 0xf3;
    local_4 = 1;
    this_00 = FUN_0053f030(local_14,0,1);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_0053f734:
  if (this_00 != (int *)0x0) {
    FUN_0053a4f0(this_00,this,(int *)&param_1);
  }
  ExceptionList = local_c;
  return bVar2;
}


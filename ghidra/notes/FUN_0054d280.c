
uint __thiscall FUN_0054d280(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  void *local_18;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648260;
  local_c = ExceptionList;
  uVar2 = 1;
  local_18 = (void *)0x0;
  local_14[0] = 1;
  local_14[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)param_1 + 4) == -1) {
    local_14[1] = 0x12;
    local_14[0] = 1;
    local_4 = 1;
    uVar2 = FUN_00506850((uint *)((int)this + 0x3c),&local_18,local_14,param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if ((uVar2 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
    bVar1 = FUN_00542b60(local_18,(int *)((int)this + 0x40),*(undefined4 *)((int)this + 0x50),
                         *(undefined4 *)((int)this + 0x54),(uint *)((int)this + 0x5c),
                         (int)this + 0x60,(int)this + 100,(int)this + 0x6c,param_1);
    uVar2 = CONCAT31(extraout_var,bVar1);
  }
  ExceptionList = local_c;
  return uVar2;
}



uint __thiscall FUN_0054d1a0(void *this,int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  undefined4 local_18;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648240;
  local_c = ExceptionList;
  uVar2 = 1;
  local_18 = 0;
  local_14[0] = 1;
  local_14[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (param_1[1] == -1) {
    local_14[1] = 0x12;
    local_14[0] = 1;
    local_4 = 1;
    uVar2 = FUN_00506850((uint *)((int)this + 0x3c),&local_18,local_14,param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if ((uVar2 != 0) && (param_1[1] == -1)) {
    bVar1 = FUN_005429e0((int *)((int)this + 0x40),*(undefined4 *)((int)this + 0x50),
                         *(undefined4 *)((int)this + 0x54),(int)this + 100,(int)this + 0x6c,param_1)
    ;
    uVar2 = CONCAT31(extraout_var,bVar1);
  }
  ExceptionList = local_c;
  return uVar2;
}



uint __thiscall FUN_004ffc90(void *this,int param_1,undefined4 param_2,int *param_3)

{
  int *this_00;
  bool bVar1;
  uint uVar2;
  uint *puVar3;
  undefined3 extraout_var;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006406e0;
  pvStack_c = ExceptionList;
  uVar2 = *(uint *)(param_1 + 8) >> 0x18;
  if ((uVar2 < 8) || (0xf < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &pvStack_c;
  FUN_00619730();
  this_00 = param_3;
  if (bVar1) {
    FUN_004fcd00(local_2c,this,3);
    local_4 = 0;
    uVar2 = FUN_00553350((uint)local_2c,param_3,1,*(uint *)((int)this + 0x24) >> 6 & 3);
    local_4 = 0xffffffff;
    FUN_004fcd80(local_2c);
  }
  else {
    (**(code **)(*param_3 + 4))();
    puVar3 = FUN_004025b0(this,(uint *)&param_3);
    local_4 = 1;
    bVar1 = FUN_004f44b0(this_00,puVar3,0);
    uVar2 = CONCAT31(extraout_var,bVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return uVar2;
}


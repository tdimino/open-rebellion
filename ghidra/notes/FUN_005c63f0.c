
void * __thiscall FUN_005c63f0(void *this,uint param_1,undefined4 param_2)

{
  undefined4 *puVar1;
  int iVar2;
  void *this_00;
  int *piVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653cfe;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005c9fb0(this,param_1,param_2);
  *(undefined ***)this = &PTR_FUN_0066bee0;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066bf58;
  *(undefined4 *)((int)this + 0xbc) = 4;
  *(undefined4 *)((int)this + 0xc0) = 0;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 1;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
  }
  local_4._0_1_ = 0;
  iVar2 = FUN_005a6340(DAT_006bc470,param_1);
  if (iVar2 != 0) {
    this_00 = (void *)FUN_00618b70(0x144);
    local_4._0_1_ = 2;
    if (this_00 == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = FUN_005c72b0(this_00,param_1);
    }
    *(int **)((int)this + 0x54) = piVar3;
    local_4._0_1_ = 0;
    (**(code **)(*piVar3 + 0x1c))();
  }
  *(undefined4 *)((int)this + 0x3c) = 2;
  *(undefined4 *)((int)this + 0x38) = 2;
  *(undefined4 *)((int)this + 0x58) = 3;
  *(undefined4 *)((int)this + 0x5c) = 3;
  *(undefined4 *)((int)this + 0x68) = 0;
  FUN_005c7130(this,0);
  ExceptionList = local_c;
  return this;
}


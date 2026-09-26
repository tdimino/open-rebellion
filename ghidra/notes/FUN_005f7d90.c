
undefined4 __thiscall
FUN_005f7d90(void *this,undefined4 *param_1,uint param_2,byte param_3,int param_4)

{
  void *this_00;
  void *this_01;
  int iVar1;
  int iVar2;
  undefined4 *this_02;
  undefined4 local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065593b;
  local_c = ExceptionList;
  this_02 = (undefined4 *)0x0;
  local_18 = 0;
  if (1 < *(uint *)((int)this + 0x94)) {
    ExceptionList = &local_c;
    this_00 = (void *)FUN_005f7cc0();
    this_01 = (void *)FUN_00618b70(0x24);
    local_4 = 0;
    if (this_01 != (void *)0x0) {
      this_02 = FUN_005faed0(this_01,param_1,param_2,param_3,param_4);
    }
    local_4 = 0xffffffff;
    iVar1 = FUN_005f5500(this_00,this_02[8]);
    while ((iVar1 != 0 && (*(int *)((int)this + 0x108) != 0))) {
      iVar2 = FUN_005f91c0(this,iVar1,param_4);
      if (iVar2 != 0) {
        ExceptionList = local_c;
        return 0;
      }
      if (*(int *)((int)this + 0x108) == 0) {
        ExceptionList = local_c;
        return 1;
      }
      iVar1 = *(int *)(iVar1 + 0x10);
    }
    if (param_4 == 0) {
      if (this_02 != (undefined4 *)0x0) {
        (**(code **)*this_02)(1);
      }
    }
    else {
      FUN_005f5600(this_02,this_02[6]);
      local_18 = this_02[6];
      FUN_005f5440((void *)((int)this_00 + 0x84),this_02);
    }
  }
  ExceptionList = local_c;
  return local_18;
}


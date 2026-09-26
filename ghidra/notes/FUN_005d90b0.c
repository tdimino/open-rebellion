
void * __thiscall
FUN_005d90b0(void *this,undefined4 param_1,undefined4 param_2,undefined4 *param_3,undefined4 param_4
            ,undefined4 param_5)

{
  int iVar1;
  void *pvVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654ac8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005d8980(this,param_2,param_1);
  *(undefined ***)this = &PTR_FUN_0066d3b0;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066d3a0;
  *(undefined4 *)((int)this + 0x3c) = param_5;
  *(undefined4 *)((int)this + 0x40) = 0;
  pvVar2 = (void *)param_3[1];
  local_4 = 0;
  iVar1 = (**(code **)(**(int **)((int)this + 0x2c) + 0x3c))
                    (*(int **)((int)this + 0x2c),0,*param_3,pvVar2,param_3[2],0);
  FUN_005d8b00(iVar1);
  *(undefined4 *)((int)this + 0x38) = 0;
  if (local_4 == 1) {
    FUN_005d9250(this);
    ExceptionList = pvVar2;
    return this;
  }
  if (local_4 != 2) {
    if (local_4 != 3) {
      FUN_005d9230(this);
      ExceptionList = pvVar2;
      return this;
    }
    FUN_005d92d0(this);
    ExceptionList = pvVar2;
    return this;
  }
  FUN_005d9310(this);
  ExceptionList = pvVar2;
  return this;
}



void __thiscall FUN_006019a0(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006561ab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (*(undefined4 **)((int)this + 8) != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    (**(code **)**(undefined4 **)((int)this + 8))(1);
  }
  pvVar1 = (void *)FUN_00618b70(0xc);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005f2fc0(pvVar1,param_1);
  }
  *(void **)((int)this + 8) = pvVar1;
  if ((*(uint *)((int)this + 0x2c) & 1) == 0) {
    *(undefined4 *)((int)this + 0x1c) = 0;
  }
  if ((*(uint *)((int)this + 0x2c) & 2) == 0) {
    *(undefined4 *)((int)this + 0x20) = 0;
  }
  ExceptionList = local_c;
  return;
}


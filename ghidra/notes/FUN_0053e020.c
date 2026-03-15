
bool __thiscall FUN_0053e020(void *this,undefined4 *param_1,undefined4 *param_2)

{
  void *pvVar1;
  uint uVar2;
  bool bVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064685b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00585420(pvVar1,param_1,param_2);
  }
  local_4 = 0xffffffff;
  bVar3 = pvVar1 != (void *)0x0;
  if (bVar3) {
    uVar2 = FUN_004f57b0(this,pvVar1);
    bVar3 = uVar2 != 0;
  }
  ExceptionList = local_c;
  return bVar3;
}


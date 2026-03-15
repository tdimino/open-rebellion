
uint __thiscall FUN_00430450(void *this,void *param_1)

{
  void *pvVar1;
  uint *puVar2;
  void *pvVar3;
  uint uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cc38;
  local_c = ExceptionList;
  uVar4 = 0;
  ExceptionList = &local_c;
  puVar2 = (uint *)FUN_00403040(param_1,&param_1);
  local_4 = 0;
  pvVar3 = FUN_004f5940(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (pvVar3 == (void *)0x0) {
    uVar4 = FUN_004f57b0(this,pvVar1);
  }
  ExceptionList = local_c;
  return uVar4;
}


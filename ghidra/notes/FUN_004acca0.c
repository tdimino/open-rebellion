
void __thiscall FUN_004acca0(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  uint *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638688;
  local_c = ExceptionList;
  if (param_1 != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    puVar2 = (uint *)FUN_00403040(param_1,&param_1);
    local_4 = 0;
    FUN_004f5910(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    (**(code **)*puVar1)(1);
  }
  ExceptionList = local_c;
  return;
}


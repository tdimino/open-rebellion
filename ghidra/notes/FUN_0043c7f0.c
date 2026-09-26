
void __fastcall FUN_0043c7f0(int param_1)

{
  uint *this;
  int *piVar1;
  undefined4 *puVar2;
  uint *puVar3;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062de08;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = param_1;
  piVar1 = FUN_004f3110(*(int *)(param_1 + 0x18),(uint *)(param_1 + 0x20));
  if (piVar1 != (int *)0x0) {
    puVar2 = FUN_0042d170(piVar1,&local_10);
    this = (uint *)(param_1 + 0x2c);
    local_4 = 0;
    FUN_004f26d0(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    puVar3 = FUN_004ece40(this);
    if (puVar3 != (uint *)0x0) {
      piVar1 = FUN_004f3000(*(int *)(param_1 + 0x18),this);
      if (piVar1 != (int *)0x0) {
        *(uint *)(param_1 + 0x30) = (uint)(*(uint *)(param_1 + 0x20) == piVar1[0x1f]);
      }
    }
  }
  ExceptionList = local_c;
  return;
}


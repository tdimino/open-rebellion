
bool __thiscall FUN_00561340(void *this,uint *param_1)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint uVar4;
  undefined3 extraout_var;
  void *in_stack_00000014;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a248;
  local_c = ExceptionList;
  bVar2 = true;
  ExceptionList = &local_c;
  piVar3 = (int *)FUN_00504dc0(param_1);
  if (piVar3 != (int *)0x0) {
    local_4 = 0;
    uVar4 = (**(code **)(*piVar3 + 4))();
    if ((uVar4 < 0x34) || (0x34 < uVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      bVar2 = FUN_005617b0(this,in_stack_00000014);
      bVar2 = CONCAT31(extraout_var,bVar2) != 0;
    }
  }
  ExceptionList = local_c;
  return bVar2;
}


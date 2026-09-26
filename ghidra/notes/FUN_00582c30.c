
bool __cdecl FUN_00582c30(uint *param_1,undefined4 *param_2)

{
  bool bVar1;
  int *piVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064efc8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_2 = 0;
  piVar2 = (int *)FUN_00504dc0(param_1);
  bVar1 = false;
  if (piVar2 != (int *)0x0) {
    local_4 = 0;
    uVar3 = (**(code **)(*piVar2 + 4))();
    if ((uVar3 < 0x30) || (0x3f < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      *param_2 = piVar2;
    }
  }
  ExceptionList = local_c;
  return bVar1;
}


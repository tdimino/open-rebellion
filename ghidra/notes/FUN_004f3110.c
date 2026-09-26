
int * __cdecl FUN_004f3110(int param_1,uint *param_2)

{
  bool bVar1;
  int *piVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f258;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_0053ef00(param_2,param_1);
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
    if (!bVar1) {
      piVar2 = (int *)0x0;
    }
  }
  ExceptionList = local_c;
  return piVar2;
}


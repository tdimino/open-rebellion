
bool __thiscall
FUN_005615b0(void *this,uint *param_1,undefined4 param_2,undefined4 param_3,void *param_4)

{
  bool bVar1;
  int *piVar2;
  uint uVar3;
  undefined3 extraout_var;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a288;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_00505190(param_1);
  bVar4 = piVar2 != (int *)0x0;
  if (piVar2 != (int *)0x0) {
    local_4 = 0;
    uVar3 = (**(code **)(*piVar2 + 4))();
    if ((uVar3 < 0x34) || (0x34 < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      bVar1 = FUN_005617b0(this,param_4);
      if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar4;
}


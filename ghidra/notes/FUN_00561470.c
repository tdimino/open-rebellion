
bool __thiscall
FUN_00561470(void *this,uint *param_1,uint *param_2,undefined4 param_3,int param_4,void *param_5)

{
  bool bVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a270;
  local_c = ExceptionList;
  bVar5 = true;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_00504dc0(param_1);
  if (piVar2 != (int *)0x0) {
    local_4 = 0;
    uVar3 = (**(code **)(*piVar2 + 4))();
    if ((uVar3 < 0x20) || (0x21 < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      iVar4 = FUN_00504dc0(param_2);
      bVar5 = iVar4 != 0;
      if (iVar4 != 0) {
        bVar1 = FUN_00561780(piVar2,param_4,param_5);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar5)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
      }
    }
    bVar1 = true;
    local_4 = 1;
    uVar3 = (**(code **)(*piVar2 + 4))();
    if ((uVar3 < 0x30) || (0x3b < uVar3)) {
      bVar1 = false;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((bVar1) && (param_4 != 0)) {
      bVar1 = FUN_005618d0(this,piVar2,param_5);
      if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar5;
}


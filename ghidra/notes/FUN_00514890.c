
int __cdecl FUN_00514890(uint *param_1,void *param_2,int param_3,void *param_4)

{
  uint *puVar1;
  void *pvVar2;
  bool bVar3;
  int *piVar4;
  void *this;
  uint uVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar6;
  int iVar7;
  bool bVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006425e8;
  local_c = ExceptionList;
  iVar7 = 1;
  ExceptionList = &local_c;
  piVar4 = (int *)FUN_00504dc0(param_1);
  if (piVar4 != (int *)0x0) {
    this = (void *)thunk_FUN_00506e40();
    pvVar2 = param_2;
    bVar8 = this != (void *)0x0;
    if (this != (void *)0x0) {
      iVar7 = FUN_00545670(this,puVar1,param_2,param_3,param_4);
      if ((iVar7 == 0) || (!bVar8)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
    }
    iVar7 = FUN_005175e0(piVar4,param_4);
    if ((iVar7 == 0) || (!bVar8)) {
      iVar7 = 0;
    }
    else {
      iVar7 = 1;
    }
    local_4 = 0;
    uVar5 = (**(code **)(*piVar4 + 4))();
    if ((uVar5 < 0x10) || (0x3f < uVar5)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar8) {
      param_1 = (uint *)0x0;
      param_2 = (void *)0x0;
      bVar8 = FUN_005187a0((int)pvVar2,(int *)&param_1);
      if ((CONCAT31(extraout_var,bVar8) == 0) || (iVar7 == 0)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
      bVar3 = FUN_005187a0(param_3,(int *)&param_2);
      if ((CONCAT31(extraout_var_00,bVar3) == 0) || (!bVar8)) {
        iVar7 = 0;
      }
      else {
        iVar7 = 1;
      }
      if (param_1 != (uint *)0x0) {
        iVar6 = FUN_0052fff0(param_1,param_4);
        if ((iVar6 == 0) || (iVar7 == 0)) {
          iVar7 = 0;
        }
        else {
          iVar7 = 1;
        }
      }
      if (param_2 != (void *)0x0) {
        iVar6 = FUN_0052fff0(param_2,param_4);
        if ((iVar6 == 0) || (iVar7 == 0)) {
          iVar7 = 0;
        }
        else {
          iVar7 = 1;
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar7;
}


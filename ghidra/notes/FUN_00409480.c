
void __thiscall FUN_00409480(void *this,int *param_1)

{
  int *piVar1;
  int *piVar2;
  bool bVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar7;
  
  piVar1 = (int *)((int)this + 4);
  piVar2 = (int *)((int)this + 8);
  *piVar1 = 1;
  *piVar2 = 0;
  if (DAT_006b1218 != (undefined4 *)0x0) {
    (**(code **)*DAT_006b1218)(1);
    DAT_006b1218 = (undefined4 *)0x0;
  }
  FUN_005975a0();
  FUN_005f4d40(param_1);
  FUN_005f4d90(param_1,piVar1);
  FUN_005f4d90(param_1,piVar2);
  FUN_004fcdb0((void *)((int)this + 0xc),param_1);
  FUN_004fcdb0((void *)((int)this + 0x10),param_1);
  iVar7 = *piVar1;
  if (iVar7 != 0) {
    if (*piVar2 == 2) {
      uVar4 = FUN_0040ac40();
      iVar7 = 0;
      if (uVar4 != 0) {
        FUN_005f4d40(param_1);
        if (DAT_006b1218 != (undefined4 *)0x0) {
          FUN_0041dd60(DAT_006b1218,param_1);
        }
        FUN_005f4d40(param_1);
        FUN_00401270(param_1);
        FUN_005f4d40(param_1);
        *piVar1 = 1;
        return;
      }
    }
    else if ((iVar7 != 0) && (*piVar2 == 0x14)) {
      iVar5 = FUN_00597530();
      iVar7 = 0;
      if (*(int *)(iVar5 + 0x694) != 0) {
        iVar5 = FUN_00597530();
        iVar7 = 0;
        if (*(int *)(iVar5 + 0x694) != 0) {
          iVar6 = FUN_0051ebb0();
          iVar7 = 0;
          if (iVar6 != 0) {
            bVar3 = FUN_004012b0(param_1);
            iVar7 = 0;
            if (CONCAT31(extraout_var,bVar3) != 0) {
              bVar3 = FUN_00409a50(param_1);
              iVar7 = 0;
              if (CONCAT31(extraout_var_00,bVar3) != 0) {
                iVar7 = FUN_0059a4f0(iVar5);
              }
            }
          }
        }
      }
    }
  }
  *piVar1 = iVar7;
  return;
}


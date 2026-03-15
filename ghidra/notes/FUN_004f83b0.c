
int __cdecl FUN_004f83b0(void *param_1)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  int *piVar4;
  int *piVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  undefined3 extraout_var_05;
  undefined3 extraout_var_06;
  int iVar6;
  
  iVar6 = 1;
  if ((*(uint *)((int)param_1 + 0x50) & 0x8000) != 0) {
    piVar3 = (int *)FUN_00539fd0(param_1,0);
    piVar4 = (int *)FUN_00539fd0(param_1,1);
    piVar5 = (int *)FUN_00539fd0(param_1,2);
    bVar1 = FUN_00539ff0((int)param_1);
    iVar6 = CONCAT31(extraout_var,bVar1);
    if (iVar6 != 0) {
      bVar1 = FUN_0053f3a0(piVar3);
      bVar2 = FUN_0053f3a0(piVar4);
      if ((CONCAT31(extraout_var_01,bVar2) == 0) || (CONCAT31(extraout_var_00,bVar1) == 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      bVar2 = FUN_0053f3a0(piVar5);
      if ((CONCAT31(extraout_var_02,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      bVar2 = FUN_0053f470(piVar3);
      if ((CONCAT31(extraout_var_03,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      bVar2 = FUN_0053f870(piVar3);
      if ((CONCAT31(extraout_var_04,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      bVar2 = FUN_0053f870(piVar4);
      if ((CONCAT31(extraout_var_05,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      bVar2 = FUN_0053f870(piVar5);
      if ((CONCAT31(extraout_var_06,bVar2) == 0) || (!bVar1)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
      if (piVar3 != (int *)0x0) {
        (**(code **)*piVar3)(1);
      }
      if (piVar4 != (int *)0x0) {
        (**(code **)*piVar4)(1);
      }
      if (piVar5 != (int *)0x0) {
        (**(code **)*piVar5)(1);
      }
    }
  }
  return iVar6;
}


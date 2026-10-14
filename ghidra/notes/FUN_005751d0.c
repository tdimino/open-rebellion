
bool __thiscall FUN_005751d0(int param_1,undefined4 param_2,int param_3,undefined4 param_4)

{
  int iVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  bool bVar5;
  undefined4 uVar6;
  
  uVar6 = param_4;
  iVar3 = param_3;
  iVar1 = FUN_00594460(param_2,param_3,param_4);
  bVar5 = iVar1 != 0;
  if (iVar3 == 8) {
    uVar2 = FUN_004fd340(uVar6);
    iVar1 = FUN_00574960(uVar2,uVar6);
    if ((iVar1 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  if (iVar3 != 0xb) {
    return bVar5;
  }
  iVar3 = FUN_00520ac0();
  if (iVar3 < 8) {
    return bVar5;
  }
  iVar3 = *(int *)(param_1 + 0xa8);
  if (iVar3 == 0) {
    return bVar5;
  }
  iVar1 = FUN_004fd340();
  param_3 = 0;
  iVar4 = FUN_00507010();
  if ((iVar4 == 0) || (bVar5 == false)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  if (iVar4 == 0) {
    return bVar5;
  }
  if (*(int *)(param_1 + 0x60) == 3) {
    iVar3 = FUN_0055ca60((int)*(short *)(iVar4 + 0x8c),&param_3);
    if ((iVar3 != 0) && (bVar5 != false)) {
      bVar5 = true;
      goto LAB_005752bf;
    }
  }
  else {
    iVar3 = FUN_0055ca90((int)*(short *)(iVar4 + 0x8c),iVar1 - iVar3,&param_3);
    if ((iVar3 != 0) && (bVar5 != false)) {
      bVar5 = true;
      goto LAB_005752bf;
    }
  }
  bVar5 = false;
LAB_005752bf:
  uVar6 = param_4;
  iVar3 = FUN_004ee150((*(short *)(iVar4 + 0x8e) + param_3) - (int)*(short *)(iVar4 + 0x8c),param_4)
  ;
  if ((iVar3 == 0) || (!bVar5)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  iVar3 = FUN_0056f730(1,uVar6);
  if ((iVar3 != 0) && (bVar5)) {
    return true;
  }
  return false;
}


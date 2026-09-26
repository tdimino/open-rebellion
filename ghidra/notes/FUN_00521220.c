
undefined4 __thiscall
FUN_00521220(void *this,void *param_1,undefined4 *param_2,int *param_3,undefined4 *param_4)

{
  undefined4 *puVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  
  puVar1 = param_2;
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  bVar2 = FUN_00520e40((int)this);
  if ((CONCAT31(extraout_var,bVar2) == 0) && ((*(byte *)((int)param_1 + 0x78) & 0xc) == 0)) {
    return 1;
  }
  bVar2 = false;
  param_2 = (undefined4 *)0x0;
  FUN_004f6b50(param_1,(int *)&param_2);
  if (param_2 != (undefined4 *)0x0) {
    bVar2 = (((byte)*(undefined4 *)((int)param_1 + 0x24) ^ (byte)param_2[9]) & 0xc0) == 0;
  }
  *puVar1 = 1;
  if ((((*(uint *)((int)param_1 + 0x50) & 0x800) == 0) &&
      (iVar3 = FUN_00520ba0((int)this), iVar3 != 2)) &&
     ((iVar3 = FUN_00520ba0((int)this), iVar3 != 1 || (bVar2)))) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  *param_3 = iVar3;
  if ((iVar3 != 0) && (iVar3 = FUN_00520ba0((int)this), iVar3 != 2)) {
    *param_4 = 1;
    return 1;
  }
  *param_4 = 0;
  return 1;
}


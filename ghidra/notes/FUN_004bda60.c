
int * __thiscall FUN_004bda60(int *param_1,int *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  *param_2 = 0;
  switch(param_1[7]) {
  case 1:
    if ((param_1[8] & 0x60000000U) == 0) {
      if ((param_1[8] & 0x10000000U) == 0) {
        param_1[7] = 2;
      }
      else {
        FUN_0047aff0(param_1);
        param_1[7] = 0;
        *param_2 = 1;
      }
    }
    else {
      piVar3 = FUN_0047b250(param_1);
      param_1[7] = 0;
      *param_2 = 1;
    }
    break;
  case 2:
    bVar1 = FUN_004bdbc0((int)param_1);
    param_1[7] = (-(uint)(CONCAT31(extraout_var,bVar1) != 0) & 0xfffffffa) + 9;
    break;
  case 3:
    iVar2 = FUN_004bdcd0((int)param_1);
    param_1[7] = (-(uint)(iVar2 != 0) & 0xfffffffb) + 9;
    break;
  case 4:
    FUN_0047b110(param_1);
    param_1[7] = 5;
    break;
  case 5:
    (**(code **)(*param_1 + 0x40))();
    param_1[7] = 6;
    break;
  case 6:
    (**(code **)(*param_1 + 0x44))();
    param_1[7] = 7;
    break;
  case 7:
    piVar3 = FUN_004be230((int)param_1);
    if ((*(byte *)(param_1 + 8) & 8) != 0) {
      param_1[7] = 8;
      break;
    }
    goto LAB_004bdb67;
  case 8:
    piVar3 = FUN_0047b360((int)param_1);
    param_1[7] = 0;
    *param_2 = 1;
    break;
  case 9:
    FUN_004be4c0(param_1);
LAB_004bdb67:
    param_1[7] = 0;
    *param_2 = 1;
    break;
  default:
    param_1[7] = 1;
  }
  if (*param_2 != 0) {
    param_1[7] = 0;
  }
  return piVar3;
}


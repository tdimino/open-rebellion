
UINT FUN_0045d8f0(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  int iVar1;
  UINT UVar2;
  
  UVar2 = 0;
  switch(param_2) {
  case 0x407:
    if (((short)param_3 == 100) && (param_1[0x52] != 0)) {
      FUN_0045f480(param_1,2);
      return 0;
    }
    break;
  case 0x408:
    if (param_3 == (HPALETTE)&DAT_00000064) {
      iVar1 = FUN_00609650((void *)param_1[0x4b],param_1[0x4a] + 0x98,0,0);
      iVar1 = FUN_006098f0((void *)param_1[0x4b],iVar1);
      param_1[0x52] = iVar1;
      return 0;
    }
    break;
  default:
    UVar2 = FUN_00606650(param_1,param_2,param_3,param_4);
    break;
  case 0x40d:
    (**(code **)(*param_1 + 0x30))();
    return 0;
  case 0x483:
    if ((param_3 == (HPALETTE)0xbbe) && (param_1[0x45] != 1)) {
      FUN_0060d7e0((void *)param_1[0x47],0x68,1);
      return 0;
    }
  }
  return UVar2;
}


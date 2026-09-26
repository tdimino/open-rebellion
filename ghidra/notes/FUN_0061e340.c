
uint __cdecl FUN_0061e340(uint param_1,int *param_2)

{
  uint uVar1;
  char *pcVar2;
  int *piVar3;
  byte bVar4;
  undefined3 extraout_var;
  undefined *puVar5;
  uint uVar6;
  uint uVar7;
  
  piVar3 = param_2;
  uVar7 = param_2[3];
  uVar1 = param_2[4];
  if (((uVar7 & 0x82) == 0) || ((uVar7 & 0x40) != 0)) {
LAB_0061e463:
    param_2[3] = uVar7 | 0x20;
    return 0xffffffff;
  }
  uVar6 = 0;
  if ((uVar7 & 1) != 0) {
    param_2[1] = 0;
    if ((uVar7 & 0x10) == 0) goto LAB_0061e463;
    *param_2 = param_2[2];
    param_2[3] = uVar7 & 0xfffffffe;
  }
  uVar7 = param_2[3];
  param_2[1] = 0;
  param_2[3] = uVar7 & 0xffffffef | 2;
  if ((uVar7 & 0x10c) == 0) {
    if ((param_2 == (int *)&DAT_006acca8) || (param_2 == (int *)&DAT_006accc8)) {
      bVar4 = FUN_00624ae0(uVar1);
      if (CONCAT31(extraout_var,bVar4) != 0) goto LAB_0061e3b3;
    }
    FUN_00624a80(piVar3);
  }
LAB_0061e3b3:
  if ((piVar3[3] & 0x108U) == 0) {
    uVar7 = 1;
    uVar6 = FUN_0061b640(uVar1,(char *)&param_1,1);
  }
  else {
    pcVar2 = (char *)piVar3[2];
    uVar7 = *piVar3 - (int)pcVar2;
    *piVar3 = (int)(pcVar2 + 1);
    piVar3[1] = piVar3[6] + -1;
    if ((int)uVar7 < 1) {
      if (uVar1 == 0xffffffff) {
        puVar5 = &DAT_006acf08;
      }
      else {
        puVar5 = (undefined *)((&DAT_006bed20)[(int)uVar1 >> 5] + (uVar1 & 0x1f) * 0x24);
      }
      if ((puVar5[4] & 0x20) != 0) {
        FUN_0061bb80(uVar1,0,2);
      }
      *(undefined1 *)piVar3[2] = (undefined1)param_1;
    }
    else {
      uVar6 = FUN_0061b640(uVar1,pcVar2,uVar7);
      *(undefined1 *)piVar3[2] = (undefined1)param_1;
    }
  }
  if (uVar6 != uVar7) {
    piVar3[3] = piVar3[3] | 0x20;
    return 0xffffffff;
  }
  return param_1 & 0xff;
}


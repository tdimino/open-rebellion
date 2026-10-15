
void __thiscall
FUN_004e8290(int param_1,int *param_2,undefined4 *param_3,int *param_4,undefined4 *param_5)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  *param_5 = 0;
  iVar1 = *(int *)(param_1 + 0xc);
  iVar2 = *(int *)(iVar1 + 800);
  if (((*param_2 == 0) && (iVar2 < 2)) && (0 < *(int *)(iVar1 + 0xa4))) {
    *param_2 = 0x55;
    *param_3 = 0;
  }
  else if ((*param_4 == 0) && ((2 < iVar2 || ((*(int *)(iVar1 + 0xa4) == 0 && (0 < iVar2)))))) {
    *param_4 = 0x55;
    *param_5 = 0;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x324);
  if (((*param_2 == 0) && (uVar3 = FUN_005f4960(), (uVar3 & 0x2000000) == 0)) &&
     (iVar1 < *(int *)(*(int *)(param_1 + 0xc) + 0x2cc))) {
    *param_2 = 0x53;
    *param_3 = 1;
  }
  else if ((*param_4 == 0) &&
          ((*(int *)(*(int *)(param_1 + 0xc) + 0x2cc) < iVar1 ||
           ((uVar3 = FUN_005f4960(), (uVar3 & 0x2000000) != 0 && (0 < iVar1)))))) {
    *param_4 = 0x53;
    *param_5 = 1;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x328);
  if (((*param_2 == 0) && (uVar3 = FUN_005f4960(), (uVar3 & 0x4000000) == 0)) &&
     (iVar1 < *(int *)(*(int *)(param_1 + 0xc) + 0x2d0))) {
    *param_2 = 0x53;
    *param_3 = 2;
  }
  else if ((*param_4 == 0) &&
          ((*(int *)(*(int *)(param_1 + 0xc) + 0x2d0) < iVar1 ||
           ((uVar3 = FUN_005f4960(), (uVar3 & 0x4000000) != 0 && (0 < iVar1)))))) {
    *param_4 = 0x53;
    *param_5 = 2;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x32c);
  if (((*param_2 == 0) && (uVar3 = FUN_005f4960(), (uVar3 & 0x8000000) == 0)) &&
     (iVar1 < *(int *)(*(int *)(param_1 + 0xc) + 0x2c8))) {
    *param_2 = 0x53;
    *param_3 = 3;
  }
  else if ((*param_4 == 0) &&
          ((*(int *)(*(int *)(param_1 + 0xc) + 0x2c8) < iVar1 ||
           ((uVar3 = FUN_005f4960(), (uVar3 & 0x8000000) != 0 && (0 < iVar1)))))) {
    *param_4 = 0x53;
    *param_5 = 3;
  }
  iVar1 = *(int *)(param_1 + 0xc);
  if ((*param_2 == 0) && (*(int *)(iVar1 + 0x34c) < *(int *)(iVar1 + 0x214))) {
    *param_2 = 0x51;
    *param_3 = 0;
  }
  else if ((*param_4 == 0) && (*(int *)(iVar1 + 0x214) < *(int *)(iVar1 + 0x34c))) {
    *param_4 = 0x51;
    *param_5 = 0;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x330);
  if (((*param_2 == 0) && (uVar3 = FUN_005f4960(), (uVar3 & 0x100) != 0)) &&
     (iVar1 < *(int *)(*(int *)(param_1 + 0xc) + 0x21c) + 1)) {
    *param_2 = 0x57;
    *param_3 = 0;
  }
  else if ((*param_4 == 0) &&
          ((*(int *)(*(int *)(param_1 + 0xc) + 0x21c) + 1 < iVar1 ||
           ((uVar3 = FUN_005f4960(), (uVar3 & 0x100) == 0 && (0 < iVar1)))))) {
    *param_4 = 0x57;
    *param_5 = 0;
  }
  iVar1 = *(int *)(param_1 + 0xc);
  if ((*param_2 == 0) && (*(int *)(iVar1 + 0x350) < *(int *)(iVar1 + 0x218))) {
    *param_2 = 0x56;
    *param_3 = 0;
  }
  else if ((*param_4 == 0) && (*(int *)(iVar1 + 0x218) < *(int *)(iVar1 + 0x350))) {
    *param_4 = 0x56;
    *param_5 = 0;
  }
  if (*(int *)(param_1 + 4) == 1) {
    iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x334);
    if (((*param_2 == 0) && (iVar1 < 1)) && (uVar3 = FUN_005f4960(), (uVar3 & 4) != 0)) {
      *param_2 = 0x6a;
      *param_3 = 0;
    }
    else if ((*param_4 == 0) &&
            ((1 < iVar1 || ((uVar3 = FUN_005f4960(), (uVar3 & 4) == 0 && (0 < iVar1)))))) {
      *param_4 = 0x6a;
      *param_5 = 0;
    }
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x338);
  if ((*param_2 == 0) && (iVar1 < 1)) {
    *param_2 = 0x62;
    *param_3 = 0;
  }
  else if ((*param_4 == 0) && (1 < iVar1)) {
    *param_4 = 0x62;
    *param_5 = 0;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x340);
  if (((*param_2 == 0) && (uVar3 = FUN_005f4960(), (uVar3 & 0x200) != 0)) && (iVar1 < 1)) {
    *param_2 = 0x61;
    *param_3 = 0;
  }
  else if ((*param_4 == 0) &&
          ((1 < iVar1 || ((uVar3 = FUN_005f4960(), (uVar3 & 0x200) == 0 && (0 < iVar1)))))) {
    *param_4 = 0x61;
    *param_5 = 0;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x344);
  if ((*param_2 == 0) && (uVar3 = FUN_005f50e0(), iVar1 < (int)(uVar3 & 0xffff))) {
    *param_2 = 0x54;
    *param_3 = 5;
  }
  else if ((*param_4 == 0) && (uVar3 = FUN_005f50e0(), (int)(uVar3 & 0xffff) < iVar1)) {
    *param_4 = 0x54;
    *param_5 = 5;
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0xc) + 0x348);
  if ((*param_2 == 0) &&
     (iVar2 = *(int *)(*(int *)(param_1 + 0xc) + 0x1bc), uVar3 = FUN_005f50e0(),
     iVar1 < (int)(iVar2 - (uVar3 & 0xffff)))) {
    *param_2 = 0x52;
    *param_3 = 6;
    return;
  }
  if ((*param_4 == 0) &&
     (iVar2 = *(int *)(*(int *)(param_1 + 0xc) + 0x1bc), uVar3 = FUN_005f50e0(),
     (int)(iVar2 - (uVar3 & 0xffff)) < iVar1)) {
    *param_4 = 0x52;
    *param_5 = 6;
  }
  return;
}


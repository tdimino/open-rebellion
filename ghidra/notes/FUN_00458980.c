
void __thiscall FUN_00458980(int *param_1,int param_2,undefined4 param_3)

{
  BOOL BVar1;
  RECT *pRVar2;
  uint uVar3;
  int iVar4;
  
  switch(param_2) {
  case 0x1b:
    (**(code **)(*param_1 + 0x30))();
    return;
  case 0x25:
  case 0x27:
    iVar4 = *(int *)(param_1[0x5b] + 0x94);
    if (iVar4 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(iVar4 + 0x24);
    }
    iVar4 = FUN_00604500((void *)(param_1[0x5b] + 0x6c),uVar3);
    if (param_2 == 0x25) {
      iVar4 = FUN_005f5c60(iVar4);
    }
    else {
      iVar4 = *(int *)(iVar4 + 0x10);
    }
    if (iVar4 != 0) {
      do {
        BVar1 = IsWindowVisible(*(HWND *)(iVar4 + 0x18));
        if (BVar1 != 0) {
          FUN_0060d7e0((void *)param_1[0x5b],*(uint *)(iVar4 + 0x24),1);
          return;
        }
        if (param_2 == 0x25) {
          iVar4 = FUN_005f5c60(iVar4);
        }
        else {
          iVar4 = *(int *)(iVar4 + 0x10);
        }
      } while (iVar4 != 0);
      return;
    }
    iVar4 = FUN_005f5060(param_1[0x5b] + 0x6c);
    if (iVar4 != 0) {
      FUN_0060d7e0((void *)param_1[0x5b],*(uint *)(iVar4 + 0x24),1);
      return;
    }
    break;
  case 0x26:
  case 0x28:
    if (param_1[0x58] == 0x67) {
      if ((RECT *)param_1[0x92] == (RECT *)0x0) {
        FUN_004585a0(param_1,(RECT *)(param_1 + 0x68));
        return;
      }
      iVar4 = 0;
      pRVar2 = (RECT *)(param_1 + 0x68);
      while ((RECT *)param_1[0x92] != pRVar2) {
        iVar4 = iVar4 + 1;
        pRVar2 = (RECT *)&pRVar2[3].right;
        if (2 < iVar4) {
          return;
        }
      }
      FUN_00458410(param_1,(RECT *)0x0);
      if ((param_2 == 0x26) && (iVar4 != 0)) {
        FUN_004585a0(param_1,(RECT *)(param_1 + iVar4 * 0xe + 0x5a));
        return;
      }
      if ((param_2 == 0x28) && (iVar4 + 1 < 3)) {
        pRVar2 = (RECT *)(param_1 + iVar4 * 0xe + 0x76);
      }
      FUN_004585a0(param_1,pRVar2);
      return;
    }
  default:
    (**(code **)(*(int *)param_1[8] + 0x14))((int *)param_1[8],0x100,param_2,param_3);
  }
  return;
}


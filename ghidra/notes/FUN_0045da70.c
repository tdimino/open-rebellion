
void __thiscall FUN_0045da70(int *param_1,undefined4 param_2,uint param_3)

{
  int iVar1;
  char *pcVar2;
  short sVar3;
  tagRECT tStack_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00630ce8;
  pvStack_c = ExceptionList;
  switch(param_3 & 0xffff) {
  case 0x65:
    ExceptionList = &pvStack_c;
    FUN_0060a790(&tStack_1c,1);
    uStack_4 = 0;
    FUN_00609410((void *)param_1[0x4b],&tStack_1c);
    if (tStack_1c.right != 0) {
      iVar1 = FUN_006098f0((void *)param_1[0x4b],*(int *)(tStack_1c.right + 0xc));
      param_1[0x52] = iVar1;
      if (iVar1 != 0) {
        sVar3 = (short)(param_3 >> 0x10);
        if (sVar3 == 0x309) {
          FUN_0060d7e0((void *)param_1[0x47],0x67,1);
        }
        else if (sVar3 == 0x29b) {
          pcVar2 = (char *)FUN_00583c40(iVar1 + 0x14);
          FUN_00604f90((void *)param_1[0x4a],pcVar2);
        }
      }
    }
    uStack_4 = 0xffffffff;
    FUN_0060a810(&tStack_1c.left);
    break;
  case 0x66:
    if (param_3 >> 0x10 == 0x67) {
      if (param_1[0x52] == 0) {
        ExceptionList = &pvStack_c;
        FUN_0060d7e0((void *)param_1[0x47],0x68,1);
        ExceptionList = pvStack_c;
        return;
      }
      ExceptionList = &pvStack_c;
      FUN_0045f480(param_1,2);
      goto LAB_0045dc43;
    }
    if (param_3 >> 0x10 == 0x68) {
      ExceptionList = &pvStack_c;
      FUN_0045f480(param_1,1);
      SetFocus(*(HWND *)(param_1[0x4a] + 0x18));
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 0x6e:
    ExceptionList = &pvStack_c;
    FUN_0045f100(param_1,param_3 >> 0x10,0);
    iVar1 = FUN_00609650((void *)param_1[0x4b],param_1[0x4a] + 0x98,0,0);
    iVar1 = FUN_006098f0((void *)param_1[0x4b],iVar1);
    param_1[0x52] = iVar1;
    SetFocus(*(HWND *)(param_1[0x4b] + 0x18));
    ExceptionList = pvStack_c;
    return;
  case 0x83:
  case 0x84:
    if ((short)param_3 == 0x84) {
      ExceptionList = &pvStack_c;
      iVar1 = (**(code **)(*(int *)param_1[0x52] + 0xc))();
    }
    else {
      ExceptionList = &pvStack_c;
      iVar1 = (**(code **)(*(int *)param_1[0x52] + 0x10))();
    }
    if (iVar1 != 0) {
      SetRect(&tStack_1c,0xc,0x1f,0x19c,0xe7);
      FUN_00609500((void *)param_1[0x4b],*(int *)(iVar1 + 0xc),0);
      pcVar2 = (char *)FUN_00583c40(iVar1 + 0x14);
      FUN_00604f90((void *)param_1[0x4a],pcVar2);
      param_1[0x52] = iVar1;
      FUN_0045fa60(param_1);
      InvalidateRect((HWND)param_1[6],&tStack_1c,0);
      SetRect(&tStack_1c,0x24,0xe,0x182,0x1e);
      InvalidateRect((HWND)param_1[6],&tStack_1c,0);
    }
LAB_0045dc43:
    SetFocus(*(HWND *)(param_1[0x4f] + 0x18));
    ExceptionList = pvStack_c;
    return;
  case 0xfb:
    ExceptionList = &pvStack_c;
    (**(code **)(*param_1 + 0x30))();
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = pvStack_c;
  return;
}


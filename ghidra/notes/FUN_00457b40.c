
void __thiscall FUN_00457b40(void *this,int param_1,undefined4 param_2,int param_3)

{
  void *this_00;
  int iVar1;
  int iVar2;
  uint uVar3;
  char *pcVar4;
  int *piVar5;
  void *pvVar6;
  undefined2 local_20;
  undefined2 local_1e;
  RECT *local_1c;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  iVar2 = param_3;
  iVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630478;
  pvStack_c = ExceptionList;
  this_00 = *(void **)((int)this + param_1 * 0x38 + 0x18c);
  local_1c = (RECT *)((int)this + param_1 * 0x38 + 0x168);
  ExceptionList = &pvStack_c;
  if (param_3 != 0) {
    ExceptionList = &pvStack_c;
    uVar3 = FUN_005f3070(param_3);
    if ((short)uVar3 != 0) {
      pcVar4 = (char *)FUN_00583c40(iVar2);
      FUN_00601aa0(this_00,pcVar4);
      goto LAB_00457c62;
    }
  }
  if ((local_1c[1].left & 0xff000000U) == 0) {
    if (iVar1 == 1) {
      piVar5 = (int *)&local_20;
      local_20 = 0x1836;
      local_1e = DAT_0065d424;
    }
    else if (iVar1 == 2) {
      param_3 = CONCAT22(DAT_0065d424,0x1837);
      piVar5 = &param_3;
    }
    else {
      if (iVar1 != 3) goto LAB_00457c62;
      piVar5 = &param_1;
      param_1 = CONCAT22(DAT_0065d424,0x1838);
    }
    FUN_006019a0(this_00,piVar5);
  }
  else {
    piVar5 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),(uint *)(local_1c + 1));
    if (piVar5 != (int *)0x0) {
      pvVar6 = FUN_004f6270(piVar5,local_18);
      local_4 = 0;
      pcVar4 = (char *)FUN_00583c40((int)pvVar6);
      FUN_00601aa0(this_00,pcVar4);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
    }
  }
LAB_00457c62:
  InvalidateRect(*(HWND *)((int)this + 0x18),local_1c,0);
  ExceptionList = pvStack_c;
  return;
}


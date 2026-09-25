
void __thiscall FUN_004697b0(void *this,int param_1)

{
  int *piVar1;
  void *this_00;
  uint *puVar2;
  int iVar3;
  int *piVar4;
  HDC hDC;
  char *pcVar5;
  int iVar6;
  uint uVar7;
  undefined4 local_28 [3];
  RECT RStack_1c;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631c40;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_006037f0(7);
  iVar3 = param_1;
  if (param_1 == *(int *)((int)this + 0x11c)) {
    ExceptionList = local_c;
    return;
  }
  if (*(int *)((int)this + 0x11c) != 0) {
    FUN_0041d3d0(*(uint *)((int)this + 0x158));
  }
  *(int *)((int)this + 0x11c) = iVar3;
  if (*(int *)((int)this + 0x118) == 1) {
    iVar6 = 0x32;
  }
  else {
    iVar6 = 0x33;
  }
  puVar2 = (uint *)FUN_006076b0(this,iVar6);
  FUN_005f52c0(local_28);
  local_4 = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  switch(iVar3) {
  case 0x79:
    FUN_0041d410();
    break;
  case 0x7a:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 1;
    break;
  case 0x7b:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 0x80;
    break;
  case 0x7c:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 0x10;
    break;
  case 0x7d:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 4;
    break;
  case 0x7e:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 8;
    break;
  case 0x7f:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 0x40;
    break;
  case 0x80:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 0x100;
    break;
  case 0x81:
    FUN_0041d410();
    *(undefined4 *)((int)this + 0x158) = 0x20;
    break;
  case 0x82:
    FUN_0041d3f0();
    *(undefined4 *)((int)this + 0x158) = 0x200;
  }
  if (*(int *)((int)this + 0x118) != 1) goto LAB_00469b76;
  FUN_00468f20((int)this);
  iVar3 = (**(code **)(**(int **)((int)this + 0x16c) + 8))();
  piVar4 = (int *)FUN_00607e90(this,0x65);
  if (iVar3 == 0) {
    (**(code **)(*piVar4 + 0x10))();
  }
  else {
    (**(code **)(*piVar4 + 0xc))();
  }
  (**(code **)(*piVar4 + 4))(5);
  FUN_0060a790(&RStack_1c,1);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00609410(*(void **)((int)this + 0x168),&RStack_1c);
  piVar4 = (int *)FUN_00607e90(this,0x67);
  if ((piVar4 != (int *)0x0) && ((**(code **)(*piVar4 + 0x10))(), RStack_1c.top == 1)) {
    iVar3 = FUN_0060a860(*(void **)((int)this + 0x16c),*(int *)(RStack_1c.right + 0xc));
    piVar1 = *(int **)(iVar3 + 0x68);
    if (piVar1 != (int *)0x0) {
      iVar3 = (**(code **)(*piVar1 + 0x10))();
      if ((iVar3 == 3) ||
         ((iVar3 = (**(code **)(*piVar1 + 0x10))(), iVar3 == 5 &&
          (iVar3 = FUN_004ece60((uint *)(piVar1 + 0x17)), iVar3 != 0)))) {
        if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
          FUN_00603150(piVar4,0,0x2916);
          uVar7 = 0x2917;
        }
        else {
          FUN_00603150(piVar4,0,0x2918);
          uVar7 = 0x2919;
        }
        FUN_00603150(piVar4,1,uVar7);
        param_1 = CONCAT22(DAT_0065d424,0x8030);
      }
      else {
        iVar3 = (**(code **)(*piVar1 + 0x10))();
        if (iVar3 != 4) goto LAB_00469a87;
        if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
          FUN_00603150(piVar4,0,0x2d20);
          uVar7 = 0x2d21;
        }
        else {
          FUN_00603150(piVar4,0,0x2d47);
          uVar7 = 0x2d48;
        }
        FUN_00603150(piVar4,1,uVar7);
        param_1 = CONCAT22(DAT_0065d424,0x8031);
      }
      FUN_00600c40(this,piVar4,&param_1);
      (**(code **)(*piVar4 + 0xc))();
    }
  }
LAB_00469a87:
  if (*(uint *)((int)this + 0x158) == 0) {
    uVar7 = FUN_0041d270();
  }
  else {
    uVar7 = FUN_0041d250(*(uint *)((int)this + 0x158));
  }
  piVar4 = (int *)FUN_00607e90(this,0x66);
  if (piVar4 != (int *)0x0) {
    (**(code **)(*piVar4 + 0xc))();
    if ((~uVar7 >> 2 & 1) == 0) {
      param_1 = CONCAT22(DAT_0065d424,0x8006);
      FUN_00600c40(this,piVar4,&param_1);
      if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) {
        uVar7 = 0x2ac9;
        goto LAB_00469b52;
      }
      FUN_00603150(piVar4,0,0x2ac7);
      uVar7 = 0x2a79;
    }
    else {
      param_1 = CONCAT22(DAT_0065d424,0x8005);
      FUN_00600c40(this,piVar4,&param_1);
      if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
        FUN_00603150(piVar4,0,0x2a78);
        uVar7 = 0x2a79;
      }
      else {
        uVar7 = 0x2a7a;
LAB_00469b52:
        FUN_00603150(piVar4,0,uVar7);
        uVar7 = 0x2a7b;
      }
    }
    FUN_00603150(piVar4,1,uVar7);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_0060a810(&RStack_1c.left);
LAB_00469b76:
  if (*(int *)(*(int *)((int)this + 0x168) + 0x3c) != 0xc2) {
    RStack_1c.left = 0x16;
    RStack_1c.top = 0x6c;
    iVar3 = FUN_005fc0e0(*(int **)((int)this + 0x164));
    RStack_1c.right = iVar3 + 0x16;
    iVar3 = FUN_005fc0f0(*(int **)((int)this + 0x164));
    RStack_1c.bottom = iVar3 + 0x6c;
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x168) + 0x18),(HWND)0x0,0,0,0x170,0xc2,6);
    this_00 = *(void **)((int)this + 0x168);
    FUN_00608300(this_00,puVar2,*(int *)((int)this_00 + 0x28),*(uint *)((int)this_00 + 0x2c));
    InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_1c,0);
  }
  hDC = GetDC(*(HWND *)((int)this + 0x18));
  RStack_1c.left = *(LONG *)((int)this + 0x1bc);
  RStack_1c.top = *(LONG *)((int)this + 0x1c0);
  RStack_1c.right = *(LONG *)((int)this + 0x1c4);
  RStack_1c.bottom = *(LONG *)((int)this + 0x1c8);
  InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_1c,0);
  iVar3 = *(int *)(*(int *)((int)this + 0x160) + 0x94);
  if (iVar3 == 0) {
    uVar7 = 0;
  }
  else {
    uVar7 = *(uint *)(iVar3 + 0x24);
  }
  iVar3 = FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),uVar7);
  pcVar5 = (char *)FUN_00602830(iVar3);
  FUN_00601aa0((void *)((int)this + 0x1a8),pcVar5);
  FUN_00601b80((void *)((int)this + 0x1a8),hDC);
  RStack_1c.top = *(LONG *)((int)this + 0x1c0);
  RStack_1c.right = *(LONG *)((int)this + 0x1c4);
  RStack_1c.left = *(LONG *)((int)this + 0x1bc);
  RStack_1c.bottom = *(LONG *)((int)this + 0x1c8);
  InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_1c,0);
  ReleaseDC(*(HWND *)((int)this + 0x18),hDC);
  FUN_0060a280(*(void **)((int)this + 0x168));
  local_4 = 0xffffffff;
  FUN_005f5300(local_28);
  ExceptionList = local_c;
  return;
}


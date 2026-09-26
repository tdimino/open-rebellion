
void __thiscall FUN_0045d140(void *this,void *param_1,int param_2,int param_3)

{
  uint *puVar1;
  bool bVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  
  if (param_1 == (void *)0x0) {
    return;
  }
  uVar3 = FUN_006037f0(7);
  FUN_0045afc0(this,param_1);
  puVar1 = (uint *)((int)param_1 + 0x3c);
  uVar4 = *(uint *)((int)param_1 + 0x3c) & 0x1c;
  bVar2 = false;
  if (uVar4 == 4) {
    iVar5 = 1;
  }
  else if (uVar4 == 8) {
    iVar5 = 2;
  }
  else {
    iVar5 = (-(uint)(uVar4 != 0x10) & 0xfffffffd) + 3;
  }
  if (param_2 == iVar5) goto LAB_0045d21b;
  uVar4 = FUN_005f4960(puVar1);
  *puVar1 = uVar4 & 0xffffffe3;
  if (param_2 == 1) {
    uVar4 = *puVar1 | 4;
LAB_0045d1cc:
    *puVar1 = uVar4;
  }
  else {
    if (param_2 == 2) {
      uVar4 = *puVar1 | 8;
      goto LAB_0045d1cc;
    }
    if (param_2 == 3) {
      uVar4 = *puVar1 | 0x10;
      goto LAB_0045d1cc;
    }
  }
  if (*(int *)((int)param_1 + 0x54) != 0) {
    uVar4 = *(uint *)((int)param_1 + 0xc) >> 0x10;
    uVar6 = FUN_0045ca80(uVar4,param_2,0);
    uVar7 = FUN_0045ca80(uVar4,param_2,1);
    FUN_0060bd20(param_1,(short)uVar6,(short)uVar7,uVar3);
  }
  bVar2 = true;
LAB_0045d21b:
  if (*(int *)((int)param_1 + 0x54) != param_3) {
    bVar2 = true;
    *(int *)((int)param_1 + 0x54) = param_3;
    if (param_3 == 0) {
      FUN_00600db0(this,*(undefined4 *)((int)param_1 + 0xc));
      uVar4 = FUN_005f4960(puVar1);
      *puVar1 = uVar4 & 0xfffffffd;
    }
    else {
      *puVar1 = *puVar1 | 2;
      uVar4 = *(uint *)((int)param_1 + 0xc) >> 0x10;
      uVar6 = FUN_0045ca80(uVar4,param_2,0);
      uVar7 = FUN_0045ca80(uVar4,param_2,1);
      FUN_0060bd20(param_1,(short)uVar6,(short)uVar7,uVar3);
    }
  }
  if (bVar2) {
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)((int)param_1 + 0x40),0);
  }
  return;
}


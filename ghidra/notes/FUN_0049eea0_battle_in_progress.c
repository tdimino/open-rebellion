
void __fastcall FUN_0049eea0(void *param_1)

{
  undefined4 uVar1;
  int iVar2;
  uint *this;
  HDC pHVar3;
  HDC hdc;
  void *this_00;
  int iVar4;
  uint *puVar5;
  HGDIOBJ h;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined4 local_78;
  undefined4 local_74 [11];
  undefined4 uStack_48;
  undefined4 local_40 [13];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063732e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  FUN_00601880(local_74,(char *)0x0,0,0,0,0,0,0);
  local_4 = 0;
  iVar2 = FUN_004067d0();
  if (iVar2 == 0) {
    local_78 = (void *)CONCAT22(DAT_0065d424,0x1a00);
    FUN_006019a0(local_74,&local_78);
  }
  else {
    FUN_00601880(local_40,s_Battle_in_Progress____006a879c,0,0,0,0,0,0);
    puVar6 = local_40;
    puVar7 = local_74;
    for (iVar2 = 0xd; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar7 = *puVar6;
      puVar6 = puVar6 + 1;
      puVar7 = puVar7 + 1;
    }
    FUN_00601990((int)local_40);
  }
  local_78 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 1;
  if (local_78 == (void *)0x0) {
    this = (uint *)0x0;
  }
  else {
    this = (uint *)FUN_005fbd20(local_78,uVar1,0x2a11,10);
  }
  local_4._0_1_ = 0;
  pHVar3 = GetDC(*(HWND *)((int)param_1 + 0x18));
  hdc = CreateCompatibleDC(pHVar3);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),pHVar3);
  this_00 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 2;
  local_78 = this_00;
  if (this_00 == (void *)0x0) {
    puVar5 = (uint *)0x0;
  }
  else {
    puVar5 = this;
    pHVar3 = hdc;
    iVar2 = FUN_005fc0f0((int *)this);
    iVar4 = FUN_005fc0e0((int *)this);
    puVar5 = (uint *)FUN_005fbea0(this_00,iVar4,iVar2,puVar5,pHVar3);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005fcc30(this,puVar5,0,0,0,0,(undefined4 *)0x0,0);
  if (this != (uint *)0x0) {
    FUN_005fbfa0(this);
    FUN_00618b60((undefined *)this);
  }
  h = (HGDIOBJ)FUN_005fc130((int)puVar5);
  local_78 = SelectObject(hdc,h);
  FUN_00601b30(local_74,0,0xe);
  local_74[0] = *(undefined4 *)((int)param_1 + 0x30);
  FUN_00601c60(local_74,0xb);
  uStack_48 = 1;
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    iVar2 = 0xff;
  }
  else {
    iVar2 = 0xff00;
  }
  FUN_00601c90(local_74,iVar2,(HWND)0x0);
  FUN_00403e90(local_74,1);
  FUN_00601ce0(local_74,hdc);
  SelectObject(hdc,local_78);
  DeleteDC(hdc);
  FUN_006075b0(param_1,(int *)puVar5,1);
  iVar2 = *(int *)((int)param_1 + 0x114);
  FUN_00606980(param_1,*(int *)(iVar2 + 0xcc),*(int *)(iVar2 + 0xd0),*(int *)(iVar2 + 0xd4),
               *(int *)(iVar2 + 0xd8));
  local_4 = 0xffffffff;
  FUN_00601990((int)local_74);
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_0046a320(void *this,uint param_1,uint param_2)

{
  bool bVar1;
  uint *puVar2;
  undefined3 extraout_var;
  void *this_00;
  int iVar3;
  int iVar4;
  undefined3 extraout_var_00;
  int iVar5;
  int iVar6;
  undefined3 extraout_var_01;
  int *this_01;
  uint *puVar7;
  uint uVar8;
  undefined4 *puVar9;
  uint uVar10;
  tagRECT local_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631c9b;
  pvStack_c = ExceptionList;
  if (((short)param_2 != 0) || (ExceptionList = &pvStack_c, (short)param_1 != 0)) {
    puVar7 = (uint *)0x0;
    this_01 = (int *)0x0;
    ExceptionList = &pvStack_c;
    if ((short)param_2 != 0) {
      ExceptionList = &pvStack_c;
      puVar2 = (uint *)FUN_0046a280(this,param_2);
      if (puVar2 != (uint *)0x0) {
        bVar1 = FUN_005fc040((int *)puVar2);
        if (CONCAT31(extraout_var,bVar1) != 0) {
          this_00 = (void *)FUN_00618b70(0x28);
          puVar7 = (uint *)0x0;
          local_4 = 0;
          if (this_00 != (void *)0x0) {
            puVar7 = puVar2;
            iVar3 = FUN_005fc0f0((int *)puVar2);
            iVar4 = FUN_005fc0e0((int *)puVar2);
            puVar7 = FUN_005fbda0(this_00,iVar4,iVar3,puVar7);
          }
          local_4 = 0xffffffff;
          FUN_005fcc30(puVar2,puVar7,0,0,0,0,(undefined4 *)0x0,0);
        }
        if (puVar2 != (uint *)0x0) {
          FUN_005fbfa0(puVar2);
          FUN_00618b60((undefined *)puVar2);
        }
      }
    }
    if ((short)param_1 != 0) {
      this_01 = FUN_0046a280(this,param_1);
      if (this_01 != (int *)0x0) {
        bVar1 = FUN_005fc040(this_01);
        if (CONCAT31(extraout_var_00,bVar1) != 0) {
          iVar3 = FUN_005fc0e0(this_01);
          iVar4 = FUN_005fc0e0((int *)puVar7);
          iVar5 = FUN_005fc0f0(this_01);
          iVar6 = FUN_005fc0f0((int *)puVar7);
          FUN_005fd0f0(this_01,puVar7,(iVar4 - iVar3) / 2,(iVar6 - iVar5) / 2);
        }
      }
    }
    if (puVar7 != (uint *)0x0) {
      bVar1 = FUN_005fc040((int *)puVar7);
      if (CONCAT31(extraout_var_01,bVar1) != 0) {
        uVar10 = 0;
        puVar9 = (undefined4 *)0x0;
        uVar8 = 0;
        iVar5 = 0;
        iVar4 = 0x21;
        iVar3 = 0xc;
        puVar2 = (uint *)FUN_006073d0(this);
        FUN_005fcc30(puVar7,puVar2,iVar3,iVar4,iVar5,uVar8,puVar9,uVar10);
        FUN_005fbfa0(puVar7);
        FUN_00618b60((undefined *)puVar7);
        if (this_01 != (int *)0x0) {
          FUN_005fbfa0(this_01);
          FUN_00618b60((undefined *)this_01);
        }
      }
    }
  }
  SetRect(&local_1c,0,0,0x19c,0x14b);
  InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
  ExceptionList = pvStack_c;
  return;
}


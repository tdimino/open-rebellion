
undefined4 __thiscall FUN_00487cc0(void *this,int *param_1)

{
  uint uVar1;
  bool bVar2;
  int iVar3;
  undefined4 uVar4;
  void *pvVar5;
  uint *puVar6;
  int *piVar7;
  undefined4 unaff_EBX;
  uint unaff_retaddr;
  uint local_24 [4];
  undefined4 local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634a88;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_24[0] = 0;
  ExceptionList = &pvStack_c;
  FUN_00520570(&local_14);
  local_4 = 0;
  iVar3 = (**(code **)(*param_1 + 0x1c))(&local_14);
  if (iVar3 != 0) {
    uVar4 = (**(code **)(*param_1 + 0xc))();
    switch(uVar4) {
    case 0x201:
      iVar3 = (**(code **)(*param_1 + 0x28))();
      pvVar5 = (void *)thunk_FUN_005f5060(iVar3);
      if (pvVar5 == (void *)0x0) break;
      puVar6 = FUN_00403040(pvVar5,&stack0x00000000);
      puStack_8._0_1_ = 1;
      piVar7 = FUN_004f2d10(*(int *)((int)this + 0x24),puVar6);
      puStack_8._0_1_ = 0;
      FUN_00619730();
      pvVar5 = (void *)0x0;
      if (piVar7 != (int *)0x0) {
        pvVar5 = (void *)piVar7[7];
        unaff_retaddr = (uint)piVar7[9] >> 6 & 3;
      }
      if (pvVar5 == (void *)0x0) break;
      uVar1 = *(uint *)((int)pvVar5 + 0x24);
      puVar6 = FUN_004025b0(pvVar5,local_24);
      local_24[1] = 0x90;
      local_24[2] = 0x98;
      puStack_8._0_1_ = 2;
      if ((0x8f < *puVar6 >> 0x18) && (*puVar6 >> 0x18 < 0x98)) {
        bVar2 = true;
      }
      FUN_00619730();
      puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
      FUN_00619730();
      if (!bVar2) break;
      if (((*(byte *)((int)pvVar5 + 0x88) & 0x20) == 0) || (unaff_retaddr != (uVar1 >> 6 & 3))) {
        unaff_EBX = 0;
        break;
      }
    case 0x200:
    case 0x202:
    case 0x213:
    case 0x242:
    case 0x250:
      unaff_EBX = 1;
      break;
    default:
      unaff_EBX = 0;
    }
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return unaff_EBX;
}


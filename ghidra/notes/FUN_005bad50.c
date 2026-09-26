
undefined4 __thiscall FUN_005bad50(void *this,int param_1)

{
  ushort uVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  undefined4 *puVar6;
  int iVar7;
  ushort uStack_6;
  
  iVar3 = param_1;
  uVar2 = *(uint *)(param_1 + 0x1c);
  if (uVar2 < 0x13d) {
    uVar5 = *(uint *)((int)this + uVar2 * 4 + 0x50);
    if ((uVar5 < 2) || (4 < uVar5)) {
      iVar4 = 0;
    }
    else {
      iVar4 = uVar5 - 1;
      iVar7 = 0;
      FUN_005aaf90();
      iVar4 = FUN_005a8a70(iVar7,iVar4);
    }
    uVar1 = *(ushort *)((int)this + (iVar4 + 0x51c + uVar2 * 4) * 2);
    uStack_6 = (ushort)((uint)*(undefined4 *)(param_1 + 0x28) >> 0x10);
    param_1 = CONCAT22(uStack_6,uVar1);
    *(int *)(iVar3 + 0x28) = param_1;
    uVar2 = *(uint *)((int)this + uVar2 * 4 + 0x544);
    if (uVar2 < 5) {
      uVar5 = FUN_005bad00(this,uVar2);
      if (uVar5 < *(uint *)((int)this + uVar2 * 4 + 0x1c)) {
        if (*(int *)(iVar3 + 0x20) == 0) {
          *(undefined4 *)(iVar3 + 0x20) = 1;
          uVar2 = *(uint *)(iVar3 + 0x3c);
          puVar6 = (undefined4 *)(iVar3 + 0x2c);
          for (iVar4 = 6; iVar4 != 0; iVar4 = iVar4 + -1) {
            *puVar6 = 0;
            puVar6 = puVar6 + 1;
          }
          *(uint *)(iVar3 + 0x3c) = (uVar2 >> 3 & 1) << 3 | *(uint *)(iVar3 + 0x3c) & 0xfffffff1 | 1
          ;
          *(uint *)(iVar3 + 0x30) = (uint)uVar1;
          *(uint *)(iVar3 + 0x34) = (uint)uStack_6;
          FUN_00610c30(iVar3 + 0x2c);
          return 1;
        }
      }
      else {
        *(undefined4 *)(iVar3 + 0x20) = 0;
      }
    }
  }
  return 0;
}


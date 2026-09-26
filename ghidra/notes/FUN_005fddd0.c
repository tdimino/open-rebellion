
undefined4 __thiscall FUN_005fddd0(void *this,undefined *param_1)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  undefined *puVar5;
  uint uVar6;
  void *this_00;
  undefined4 uVar7;
  int iVar8;
  int iVar9;
  byte *pbVar10;
  byte *pbVar11;
  undefined4 *puVar12;
  undefined4 *puVar13;
  int iVar14;
  int iVar15;
  uint uStack_38;
  uint uStack_34;
  byte *pbStack_30;
  int *local_28;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655f2b;
  local_c = ExceptionList;
  iVar15 = 0;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x24) = 0;
  uVar6 = FUN_005fc100(this);
  if (uVar6 == 8) {
    this_00 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      local_28 = (int *)0x0;
    }
    else {
      local_28 = FUN_005fbda0(this_00,0x100,2,this);
    }
    local_4 = 0xffffffff;
    if (local_28 != (int *)0x0) {
      iVar4 = *(int *)this;
      iVar8 = *local_28;
      pbVar11 = (byte *)(iVar4 + 0x28);
      puVar12 = (undefined4 *)(iVar8 + 0x28);
      if (0 < *(int *)((int)this + 0x10)) {
        puVar13 = puVar12;
        do {
          uVar7 = (*(code *)param_1)(*(undefined4 *)(((int)pbVar11 - (int)puVar12) + (int)puVar13));
          *puVar13 = uVar7;
          iVar15 = iVar15 + 1;
          puVar13 = puVar13 + 1;
        } while (iVar15 < *(int *)((int)this + 0x10));
      }
      iVar15 = FUN_00618b70(0x100);
      *(int *)((int)this + 0x24) = iVar15;
      if (iVar15 != 0) {
        uStack_34 = 0;
        pbVar10 = (byte *)(iVar8 + 0x29);
        pbStack_30 = pbVar11;
        do {
          bVar1 = *pbVar10;
          param_1 = (undefined *)(uStack_34 & 0xff);
          puVar5 = param_1;
          param_1._0_1_ = (undefined1)uStack_34;
          bVar2 = pbVar10[1];
          iVar9 = (uint)*(byte *)(iVar4 + 0x29 + (uStack_34 & 0xff) * 4) - (uint)bVar1;
          bVar3 = pbVar10[-1];
          iVar8 = (uint)pbVar11[(uStack_34 & 0xff) * 4] - (uint)bVar3;
          iVar15 = (uint)(pbVar11 + (uStack_34 & 0xff) * 4)[2] - (uint)bVar2;
          iVar15 = iVar8 * iVar8 + iVar9 * iVar9 + iVar15 * iVar15;
          if (((pbVar10[((int)pbVar11 - (int)puVar12) + 1] != bVar2) ||
              (pbVar10[(int)pbVar11 - (int)puVar12] != bVar1)) || (*pbStack_30 != bVar3)) {
            uVar6 = 0;
            uStack_38 = 0;
            if (*(uint *)((int)this + 0x10) != 0) {
              do {
                param_1 = puVar5;
                iVar14 = (uint)*(byte *)(iVar4 + 0x29 + uVar6 * 4) - (uint)bVar1;
                iVar8 = (uint)pbVar11[uVar6 * 4] - (uint)bVar3;
                iVar9 = (uint)*(byte *)(iVar4 + 0x2a + uVar6 * 4) - (uint)bVar2;
                iVar8 = iVar8 * iVar8 + iVar14 * iVar14 + iVar9 * iVar9;
                if (iVar8 < iVar15) {
                  param_1 = (undefined *)(uStack_38 & 0xff);
                  iVar15 = iVar8;
                }
                if (iVar8 == 0) break;
                uStack_38 = uStack_38 + 1;
                uVar6 = uStack_38 & 0xffff;
                puVar5 = param_1;
              } while (uVar6 < *(uint *)((int)this + 0x10));
            }
          }
          *(undefined1 *)(uStack_34 + *(int *)((int)this + 0x24)) = param_1._0_1_;
          uStack_34 = uStack_34 + 1;
          pbStack_30 = pbStack_30 + 4;
          pbVar10 = pbVar10 + 4;
        } while ((int)uStack_34 < 0x100);
      }
      FUN_005fbfa0(local_28);
      FUN_00618b60((undefined *)local_28);
    }
  }
  ExceptionList = local_c;
  return *(undefined4 *)((int)this + 0x24);
}


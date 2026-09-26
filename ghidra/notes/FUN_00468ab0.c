
undefined4 __thiscall FUN_00468ab0(void *this,int param_1)

{
  uint uVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  int iVar4;
  void *pvVar5;
  uint uVar6;
  void *pvVar7;
  int iVar8;
  int iVar9;
  uint *puVar10;
  uint *puVar11;
  undefined4 local_68 [3];
  uint local_5c [10];
  uint local_34 [10];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631bff;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_006037f0(7);
  FUN_005f52c0(local_68);
  local_4 = 0;
  FUN_005fbd20(local_34,uVar2,(*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) + 0x2aa2,10);
  local_4._0_1_ = 1;
  if (param_1 == 0) {
    if (*(int *)((int)this + 0x170) != 0) {
      puVar3 = (undefined4 *)FUN_00618b70(0x1c);
      local_4._0_1_ = 3;
      if (puVar3 == (undefined4 *)0x0) {
        puVar3 = (undefined4 *)0x0;
      }
      else {
        puVar3 = FUN_005f5590(puVar3);
      }
      local_4._0_1_ = 1;
      FUN_005f5600(puVar3,*(undefined4 *)(*(int *)((int)this + 0x170) + 0xc));
      FUN_005f5440(local_68,puVar3);
    }
  }
  else {
    for (iVar4 = *(int *)(*(int *)((int)this + 0x16c) + 8); iVar4 != 0; iVar4 = *(int *)(iVar4 + 8))
    {
      if ((*(byte *)(iVar4 + 0x3c) & 1) != 0) {
        puVar3 = (undefined4 *)FUN_00618b70(0x1c);
        local_4._0_1_ = 2;
        if (puVar3 == (undefined4 *)0x0) {
          puVar3 = (undefined4 *)0x0;
        }
        else {
          puVar3 = FUN_005f5590(puVar3);
        }
        local_4._0_1_ = 1;
        FUN_005f5600(puVar3,*(undefined4 *)(iVar4 + 0xc));
        FUN_005f5440(local_68,puVar3);
      }
    }
  }
  FUN_005f5b20(*(void **)((int)this + 0x16c));
  FUN_006082c0(*(void **)((int)this + 0x168),0x15a,0x15);
  iVar4 = FUN_0041d1b0();
  for (iVar4 = thunk_FUN_005f5080(iVar4); iVar4 != 0; iVar4 = FUN_005f5c60(iVar4)) {
    pvVar5 = (void *)FUN_00618b70(0x70);
    local_4._0_1_ = 4;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_004ad6b0(pvVar5,iVar4);
    }
    uVar1 = *(uint *)(iVar4 + 0x34);
    uVar6 = 0;
    local_4._0_1_ = 1;
    if ((uVar1 & 1) == 0) {
      if ((uVar1 & 0x40) == 0) {
        if ((uVar1 & 8) == 0) {
          if ((uVar1 & 0x10) == 0) {
            if ((uVar1 & 0x20) == 0) {
              if ((uVar1 & 0x80) == 0) {
                if ((uVar1 & 4) == 0) {
                  if ((uVar1 & 0x100) == 0) {
                    if ((uVar1 & 0x200) != 0) {
                      uVar6 = (*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) + 0x2ad8;
                    }
                  }
                  else {
                    uVar6 = 0x2ad7;
                  }
                }
                else {
                  uVar6 = 0x2ad6;
                }
              }
              else {
                uVar6 = (*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) + 0x2ad4;
              }
            }
            else {
              uVar6 = 0x2aa4;
            }
          }
          else {
            uVar6 = (*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) + 0x2a9e;
          }
        }
        else {
          uVar6 = (*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) + 0x2a9c;
        }
      }
      else {
        uVar6 = 0x2ad3;
      }
    }
    else {
      uVar6 = (*(int *)(*(int *)((int)this + 0x114) + 0x9c) != 1) + 0x2a9a;
    }
    if ((short)uVar6 != 0) {
      FUN_005fbd20(local_5c,uVar2,uVar6,10);
      local_4._0_1_ = 5;
      pvVar7 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 6;
      if (pvVar7 == (void *)0x0) {
        puVar10 = (uint *)0x0;
      }
      else {
        puVar10 = local_5c;
        iVar8 = FUN_005fc0f0((int *)local_5c);
        iVar9 = FUN_005fc0e0((int *)local_5c);
        puVar10 = FUN_005fbda0(pvVar7,iVar9,iVar8,puVar10);
      }
      local_4._0_1_ = 5;
      pvVar7 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 7;
      if (pvVar7 == (void *)0x0) {
        puVar11 = (uint *)0x0;
      }
      else {
        puVar11 = local_34;
        iVar8 = FUN_005fc0f0((int *)local_34);
        iVar9 = FUN_005fc0e0((int *)local_34);
        puVar11 = FUN_005fbda0(pvVar7,iVar9,iVar8,puVar11);
      }
      local_4._0_1_ = 5;
      FUN_005fd0f0(local_34,puVar11,0,0);
      FUN_005fcc30(local_5c,puVar10,1,1,0,0,(undefined4 *)0x0,0);
      FUN_005fcc30(local_5c,puVar11,1,1,0,0,(undefined4 *)0x0,0);
      FUN_0060be60(pvVar5,(int)puVar10,(int)puVar11);
      local_4._0_1_ = 1;
      FUN_005fbfa0(local_5c);
    }
    *(undefined4 *)((int)pvVar5 + 0x30) = 0x20;
    *(undefined4 *)((int)pvVar5 + 0x34) = 1;
    if ((*(byte *)(iVar4 + 0x24) & 1) == 0) {
      *(undefined4 *)((int)pvVar5 + 0x38) = 0xd;
    }
    else {
      *(undefined4 *)((int)pvVar5 + 0x38) = 10;
    }
    FUN_005f59f0(*(void **)((int)this + 0x16c),(int)pvVar5);
    iVar8 = FUN_005f5500(local_68,*(uint *)((int)pvVar5 + 0xc));
    if (iVar8 != 0) {
      FUN_00609500(*(void **)((int)this + 0x168),*(int *)((int)pvVar5 + 0xc),1);
    }
  }
  FUN_00468f20((int)this);
  if (param_1 == 0) {
    iVar4 = thunk_FUN_005f5060((int)local_68);
    if (iVar4 == 0) {
      *(undefined4 *)((int)this + 0x170) = 0;
    }
    else {
      uVar2 = FUN_0060a860(*(void **)((int)this + 0x16c),*(int *)(iVar4 + 0x18));
      *(undefined4 *)((int)this + 0x170) = uVar2;
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005fbfa0(local_34);
  local_4 = 0xffffffff;
  FUN_005f5300(local_68);
  ExceptionList = local_c;
  return 1;
}


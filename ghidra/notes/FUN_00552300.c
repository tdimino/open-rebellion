
int __thiscall
FUN_00552300(void *this,int param_1,int param_2,uint *param_3,void *param_4,void *param_5,
            undefined4 *param_6)

{
  void *this_00;
  int iVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  void *pvVar5;
  bool bVar6;
  int iVar7;
  void **ppvVar8;
  void *pvVar9;
  uint *local_78;
  void *local_74;
  uint local_70;
  uint local_6c;
  undefined4 local_68 [7];
  undefined4 auStack_4c [7];
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648e00;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece80(param_6);
  local_70 = 1;
  local_6c = 0xffffffff;
  local_4 = 0;
  FUN_00520580(param_4,&local_70);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = param_5;
  FUN_0054fb20((int)param_5);
  local_74 = (void *)0x0;
  FUN_0054fa40(local_68);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  iVar1 = FUN_005501e0(param_1);
  local_78 = (uint *)0x0;
  if (iVar1 != 0) {
    if (*(int *)((int)param_4 + 4) == -1) {
      iVar1 = FUN_00552e80(this,param_3,param_4,(int *)&local_78);
    }
    if (iVar1 != 0) {
      if (*(int *)((int)param_4 + 4) == -1) {
        iVar1 = FUN_00552b10(this,local_78,param_1,param_2,(int *)&local_74,this_00);
        if (iVar1 != 0) {
          local_70 = 0x90;
          local_6c = 0x98;
          local_4._0_1_ = 2;
          uVar2 = (**(code **)(*local_78 + 4))();
          if ((uVar2 < local_70) || (param_5 = (void *)0x1, local_6c <= uVar2)) {
            param_5 = (void *)0x0;
          }
          local_4._0_1_ = 1;
          FUN_00619730();
          if (param_5 == (void *)0x0) {
            local_70 = 8;
            local_6c = 0x10;
            local_4._0_1_ = 5;
            uVar2 = (**(code **)(*local_78 + 4))();
            if ((uVar2 < local_70) || (param_5 = (void *)0x1, local_6c <= uVar2)) {
              param_5 = (void *)0x0;
            }
            local_4._0_1_ = 1;
            FUN_00619730();
            if (param_5 == (void *)0x0) {
              local_70 = 0x14;
              local_6c = 0x1c;
              local_4._0_1_ = 7;
              uVar2 = (**(code **)(*local_78 + 4))();
              if ((uVar2 < local_70) || (local_6c <= uVar2)) {
                bVar6 = false;
              }
              else {
                bVar6 = true;
              }
              local_4._0_1_ = 1;
              FUN_00619730();
              if (!bVar6) {
                local_70 = 0x98;
                local_6c = 0xa0;
                local_4._0_1_ = 8;
                uVar2 = (**(code **)(*local_78 + 4))();
                if ((uVar2 < local_70) || (local_6c <= uVar2)) {
                  bVar6 = false;
                }
                else {
                  bVar6 = true;
                }
                local_4._0_1_ = 1;
                FUN_00619730();
                if (!bVar6) {
                  iVar1 = 0;
                }
              }
            }
            else {
              FUN_00502e30(auStack_2c,local_78,1,*(undefined4 *)((int)this + 0x18));
              local_4 = CONCAT31(local_4._1_3_,6);
              FUN_00513120((int)auStack_2c);
              while ((iStack_10 != 0 && (bVar6 = iVar1 != 0, iVar1 = 0, bVar6))) {
                ppvVar8 = &local_74;
                iVar1 = param_1;
                iVar7 = param_2;
                pvVar9 = this_00;
                pvVar5 = (void *)FUN_0052bed0((int)auStack_2c);
                iVar1 = FUN_00552b10(this,pvVar5,iVar1,iVar7,(int *)ppvVar8,pvVar9);
                FUN_005130d0((int)auStack_2c);
              }
              local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
              local_4._0_1_ = 1;
              FUN_00502eb0(auStack_2c);
            }
            goto LAB_0055275a;
          }
          param_3 = local_78;
          if (*(int *)((int)this + 0x1c) == 0x270) {
            iVar1 = FUN_00509b40(local_78,*(undefined4 *)((int)this + 0x18),(int *)&param_5);
          }
          else {
            iVar1 = FUN_005097d0(local_78,*(undefined4 *)((int)this + 0x18),(int *)&param_5);
          }
          if (iVar1 != 0) {
            if (param_5 != (void *)0x0) {
              iVar1 = FUN_00552b10(this,param_5,param_1,param_2,(int *)&local_74,this_00);
            }
            if (iVar1 != 0) {
              puVar3 = FUN_0054fc40(this_00,&local_70);
              iVar7 = puVar3[1];
              FUN_00619730();
              if (iVar7 != -1) {
                FUN_004ffef0(auStack_2c,param_3,1,*(undefined4 *)((int)this + 0x18));
                local_4 = CONCAT31(local_4._1_3_,3);
                FUN_00513120((int)auStack_2c);
                while ((iStack_10 != 0 && (iVar1 != 0))) {
                  uVar4 = FUN_0052bed0((int)auStack_2c);
                  FUN_00502e30(auStack_4c,uVar4,1,*(undefined4 *)((int)this + 0x18));
                  local_4 = CONCAT31(local_4._1_3_,4);
                  FUN_00513120((int)auStack_4c);
                  while ((iStack_30 != 0 && (bVar6 = iVar1 != 0, iVar1 = 0, bVar6))) {
                    ppvVar8 = &local_74;
                    iVar1 = param_1;
                    iVar7 = param_2;
                    pvVar9 = this_00;
                    pvVar5 = (void *)FUN_0052bed0((int)auStack_4c);
                    iVar1 = FUN_00552b10(this,pvVar5,iVar1,iVar7,(int *)ppvVar8,pvVar9);
                    FUN_005130d0((int)auStack_4c);
                  }
                  local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
                  local_4 = CONCAT31(local_4._1_3_,3);
                  FUN_00502eb0(auStack_4c);
                  FUN_005130d0((int)auStack_2c);
                }
                local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
                local_4._0_1_ = 1;
                FUN_004fff70(auStack_2c);
              }
              goto LAB_0055275a;
            }
          }
        }
      }
      else {
LAB_0055275a:
        if (iVar1 != 0) {
          if (*(int *)((int)param_4 + 4) == -1) {
            puVar3 = FUN_0054fc40(this_00,&local_70);
            local_4._0_1_ = 9;
            FUN_00520580(param_4,puVar3);
            local_4._0_1_ = 1;
            FUN_00619730();
          }
          if (iVar1 != 0) {
            if (*(int *)((int)param_4 + 4) == -1) {
              puVar3 = FUN_00403040(local_74,&param_2);
              local_4._0_1_ = 10;
              FUN_004f26d0(param_6,puVar3);
              local_4._0_1_ = 1;
              FUN_00619730();
            }
            if (iVar1 != 0) goto LAB_00552827;
          }
        }
      }
    }
  }
  local_70 = 1;
  local_6c = 1;
  local_4._0_1_ = 0xb;
  FUN_00520580(param_4,&local_70);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_00552827:
  local_4 = 0xffffffff;
  FUN_0054fa90(local_68);
  ExceptionList = pvStack_c;
  return iVar1;
}



void __thiscall FUN_005b49e0(void *this,int *param_1,int param_2)

{
  float fVar1;
  float fVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 *puVar5;
  int iVar6;
  float10 fVar7;
  float10 fVar8;
  int *unaff_retaddr;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00653421;
  pvStack_c = ExceptionList;
  iVar4 = *(int *)this;
  ExceptionList = &pvStack_c;
  uVar3 = (**(code **)(*param_1 + 0x20))();
  iVar4 = (**(code **)(iVar4 + 0x84))(uVar3);
  if (iVar4 != 4) {
    if (*(int *)((int)this + iVar4 * 0x74 + 0x180) != 0) {
      puVar5 = (undefined4 *)FUN_00618b70(0x24);
      puStack_8 = (undefined1 *)0x0;
      if (puVar5 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)0x0;
      }
      else {
        iVar6 = (**(code **)(*unaff_retaddr + 0x20))();
        fVar1 = *(float *)((int)this + iVar4 * 0x74 + 0x198);
        if (iVar6 == 0) {
          fVar2 = *(float *)((int)this + iVar4 * 0x74 + 0x1a0);
        }
        else {
          fVar2 = *(float *)((int)this + iVar4 * 0x74 + 0x19c);
        }
        fVar7 = (float10)(**(code **)(*(int *)this + 0x60))();
        iVar6 = *(int *)((int)this + iVar4 * 0x74 + 0x180);
        fVar8 = (float10)(**(code **)(*(int *)this + 0x50))();
        FUN_005f5c10(puVar5);
        puVar5[6] = 0;
        *puVar5 = &PTR_FUN_0066c678;
        puVar5[7] = (float)fVar8;
        puVar5[8] = (float)((float10)iVar6 * fVar7 * (float10)(fVar1 * fVar2));
      }
      puStack_8 = (undefined1 *)0xffffffff;
      if (param_2 != 0) {
        puVar5[8] = -(float)puVar5[8];
      }
      FUN_005b7e50(param_1,puVar5);
    }
    if (*(int *)((int)this + iVar4 * 0x74 + 0x184) != 0) {
      puVar5 = (undefined4 *)FUN_00618b70(0x24);
      puStack_8 = (undefined1 *)0x1;
      if (puVar5 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)0x0;
      }
      else {
        iVar6 = (**(code **)(*unaff_retaddr + 0x20))();
        if (iVar6 == 0) {
          fVar1 = *(float *)((int)this + iVar4 * 0x74 + 0x1c8);
        }
        else {
          fVar1 = *(float *)((int)this + iVar4 * 0x74 + 0x1c4);
        }
        fVar2 = *(float *)((int)this + iVar4 * 0x74 + 0x1c0);
        fVar7 = (float10)(**(code **)(*(int *)this + 0x60))();
        iVar6 = *(int *)((int)this + iVar4 * 0x74 + 0x184);
        fVar8 = (float10)(**(code **)(*(int *)this + 0x58))();
        FUN_005f5c10(puVar5);
        puVar5[6] = 0;
        *puVar5 = &PTR_FUN_0066c678;
        puVar5[7] = (float)fVar8;
        puVar5[8] = (float)((float10)iVar6 * fVar7 * (float10)(fVar1 * fVar2));
      }
      puStack_8 = (undefined1 *)0xffffffff;
      if (param_2 != 0) {
        puVar5[8] = -(float)puVar5[8];
      }
      FUN_005b7e50(param_1,puVar5);
    }
    if (*(int *)((int)this + iVar4 * 0x74 + 0x17c) != 0) {
      puVar5 = (undefined4 *)FUN_00618b70(0x24);
      puStack_8 = (undefined1 *)0x2;
      if (puVar5 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)0x0;
      }
      else {
        iVar6 = (**(code **)(*unaff_retaddr + 0x20))();
        if (iVar6 == 0) {
          fVar1 = *(float *)((int)this + iVar4 * 0x74 + 0x1b4);
        }
        else {
          fVar1 = *(float *)((int)this + iVar4 * 0x74 + 0x1b0);
        }
        fVar2 = *(float *)((int)this + iVar4 * 0x74 + 0x1ac);
        fVar7 = (float10)(**(code **)(*(int *)this + 0x60))();
        iVar4 = *(int *)((int)this + iVar4 * 0x74 + 0x17c);
        fVar8 = (float10)(**(code **)(*(int *)this + 0x54))();
        FUN_005f5c10(puVar5);
        puVar5[6] = 0;
        *puVar5 = &PTR_FUN_0066c678;
        puVar5[7] = (float)fVar8;
        puVar5[8] = (float)((float10)iVar4 * fVar7 * (float10)(fVar1 * fVar2));
      }
      puStack_8 = (undefined1 *)0xffffffff;
      if (param_2 != 0) {
        puVar5[8] = -(float)puVar5[8];
      }
      FUN_005b7e50(param_1,puVar5);
    }
  }
  ExceptionList = pvStack_10;
  return;
}


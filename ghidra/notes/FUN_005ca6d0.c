
void __thiscall FUN_005ca6d0(void *this,undefined4 param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  int iVar3;
  void *pvVar4;
  int iVar5;
  undefined4 *puVar6;
  uint uVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00654239;
  pvStack_c = ExceptionList;
  puVar6 = (undefined4 *)0x0;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)((int)this + 0x60) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)((int)this + 0x60))(1);
  }
  iVar5 = *(int *)((int)this + 100);
  *(undefined4 *)((int)this + 0x60) = 0;
  if (iVar5 == 1) {
    (**(code **)(*(int *)((int)this + 0x8c) + 4))();
  }
  *(undefined4 *)((int)this + 100) = param_1;
  switch(param_1) {
  default:
    goto switchD_005ca724_caseD_0;
  case 1:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    iVar5 = (**(code **)(*(int *)this + 4))();
    if (iVar5 == 3) {
      FUN_005cb1a0(this,1);
    }
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 4;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066ceb8;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 2:
    iVar3 = (**(code **)(*(int *)this + 4))();
    if (iVar3 != 4) goto LAB_005cab53;
    iVar5 = *(int *)((int)this + 0x50);
    iVar3 = FUN_005aaf90();
    if (iVar5 == *(int *)(iVar3 + 0x9ec)) {
      uVar7 = *(uint *)((int)this + 0x18);
      pvVar4 = (void *)FUN_0059ed70();
      FUN_005a0320(pvVar4,uVar7);
    }
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    (**(code **)(*(int *)((int)this + 0x8c) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 8;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005c6150(puVar1 + 6);
      puVar1[6] = &PTR_FUN_0066bdb8;
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066cdf8;
      puVar6 = puVar1;
    }
    break;
  case 3:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    (**(code **)(*(int *)((int)this + 0x8c) + 4))();
    piVar2 = (int *)FUN_00618b70(0x24);
    uStack_4 = 9;
    if (piVar2 == (int *)0x0) {
      piVar2 = (int *)0x0;
    }
    else {
      FUN_005c6150(piVar2 + 6);
      piVar2[6] = (int)&PTR_FUN_0066bdb8;
      piVar2[2] = (int)this;
      piVar2[5] = 0;
      piVar2[3] = 0;
      piVar2[4] = 0;
      piVar2[1] = 0;
      *piVar2 = (int)&PTR_FUN_0066cdc8;
    }
    *(int **)((int)this + 0x60) = piVar2;
    uStack_4 = 0xffffffff;
    (**(code **)(*piVar2 + 8))();
    ExceptionList = pvStack_c;
    return;
  case 4:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    (**(code **)(*(int *)((int)this + 0x8c) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 0;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005c6150(puVar1 + 6);
      puVar1[6] = &PTR_FUN_0066bdb8;
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066cf78;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 5:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    (**(code **)(*(int *)((int)this + 0x8c) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 1;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066cf48;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 6:
    iVar3 = (**(code **)(*(int *)this + 4))();
    if ((iVar3 == 4) && (*(int *)((int)this + 0x50) == 0)) {
      (**(code **)(*(int *)((int)this + 0x74) + 4))();
      (**(code **)(*(int *)((int)this + 0x8c) + 4))();
      pvVar4 = (void *)FUN_00618b70(0x84);
      uStack_4 = 10;
      if (pvVar4 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005d0820(pvVar4,this);
      }
      uStack_4 = 0xffffffff;
      *(void **)((int)this + 0x60) = pvVar4;
      iVar5 = FUN_005aaf90();
      *(undefined4 *)(*(int *)(iVar5 + 8) + 0x18) = 0;
      ExceptionList = pvStack_c;
      return;
    }
LAB_005cab53:
    *(int *)((int)this + 100) = iVar5;
    ExceptionList = pvStack_c;
    return;
  case 7:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 3;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066cee8;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 8:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 5;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066ce88;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 9:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 6;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066ce58;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 10:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 7;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066ce28;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
    break;
  case 0xb:
    (**(code **)(*(int *)((int)this + 0x74) + 4))();
    puVar1 = (undefined4 *)FUN_00618b70(0x24);
    uStack_4 = 2;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005a1750(puVar1 + 6);
      puVar1[5] = 0;
      puVar1[3] = 0;
      puVar1[4] = 0;
      puVar1[1] = 0;
      puVar1[2] = this;
      *puVar1 = &PTR_FUN_0066cf18;
      *(undefined4 **)((int)this + 0x60) = puVar1;
      ExceptionList = pvStack_c;
      return;
    }
  }
  *(undefined4 **)((int)this + 0x60) = puVar6;
switchD_005ca724_caseD_0:
  ExceptionList = pvStack_c;
  return;
}


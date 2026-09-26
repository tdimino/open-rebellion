
void __thiscall FUN_0059fd40(void *this,int param_1)

{
  uint uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  void *pvVar5;
  undefined4 *puVar6;
  float fVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00651cd7;
  local_c = ExceptionList;
  iVar3 = *(int *)(param_1 + 0xc);
  puVar6 = (undefined4 *)0x0;
  uVar1 = iVar3 - 2;
  ExceptionList = &local_c;
  switch(iVar3) {
  case 2:
  case 3:
  case 4:
  case 5:
  case 6:
  case 7:
  case 8:
  case 9:
    ExceptionList = &local_c;
    iVar3 = FUN_0059f5d0(this,uVar1);
    if (iVar3 == 0) {
      if (*(int *)this != 0) {
        puVar6 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x14))(uVar1);
      }
      break;
    }
    iVar3 = (**(code **)(*DAT_006bc464 + 8))();
    if (((iVar3 < 1) || (iVar3 = (**(code **)(*DAT_006bc464 + 8))(), 8 < iVar3)) ||
       (iVar3 = (**(code **)(*DAT_006bc464 + 8))(), iVar3 - 1U != uVar1)) {
      pvVar5 = (void *)FUN_00618b70(0x60);
      uStack_4 = 2;
      if (pvVar5 != (void *)0x0) {
        puVar6 = FUN_005a33d0(pvVar5,uVar1);
        uStack_4 = 0xffffffff;
        break;
      }
    }
    else {
      puVar6 = (undefined4 *)FUN_00618b70(0x54);
      uStack_4 = 1;
      if (puVar6 != (undefined4 *)0x0) {
        puVar6 = FUN_005a0c80(puVar6);
        uStack_4 = 0xffffffff;
        break;
      }
    }
    goto LAB_0059ff7e;
  case 0xb:
  case 0xc:
  case 0xd:
  case 0xe:
    uVar1 = iVar3 - 0xb;
    ExceptionList = &local_c;
    iVar3 = FUN_0059f5f0(this,uVar1);
    if (iVar3 == 0) break;
    iVar3 = (**(code **)(*DAT_006bc464 + 8))();
    if (((iVar3 < 9) || (iVar3 = (**(code **)(*DAT_006bc464 + 8))(), 0xc < iVar3)) ||
       (iVar3 = (**(code **)(*DAT_006bc464 + 8))(), iVar3 - 9U != uVar1)) {
      pvVar5 = (void *)FUN_00618b70(0x68);
      uStack_4 = 4;
      if (pvVar5 != (void *)0x0) {
        puVar6 = FUN_005a4640(pvVar5,uVar1);
        uStack_4 = 0xffffffff;
        break;
      }
    }
    else {
      puVar6 = (undefined4 *)FUN_00618b70(0x54);
      uStack_4 = 3;
      if (puVar6 != (undefined4 *)0x0) {
        puVar6 = FUN_005a0c80(puVar6);
        uStack_4 = 0xffffffff;
        break;
      }
    }
    goto LAB_0059ff7e;
  case 0xf:
    ExceptionList = &local_c;
    piVar2 = (int *)FUN_0059ed70();
    if (*piVar2 == 0) break;
    iVar3 = (**(code **)(*DAT_006bc464 + 8))();
    if (iVar3 != 0x11) {
      puVar6 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x2c))();
      break;
    }
    puVar6 = (undefined4 *)FUN_00618b70(0x54);
    uStack_4 = 0;
    if (puVar6 != (undefined4 *)0x0) {
      puVar6 = FUN_005a0c80(puVar6);
      uStack_4 = 0xffffffff;
      break;
    }
LAB_0059ff7e:
    uStack_4 = 0xffffffff;
    puVar6 = (undefined4 *)0x0;
    break;
  case 0x10:
    ExceptionList = &local_c;
    FUN_0059fbd0(this,3);
    break;
  case 0x11:
    ExceptionList = &local_c;
    FUN_0059fbd0(this,2);
    break;
  case 0x12:
    ExceptionList = &local_c;
    FUN_0059fbd0(this,1);
    break;
  case 0x13:
    ExceptionList = &local_c;
    FUN_0059fbd0(this,0);
    break;
  case 0x14:
    ExceptionList = &local_c;
    FUN_0059ef40((int)this);
    break;
  case 0x15:
    ExceptionList = &local_c;
    FUN_0059ef20((int)this);
    break;
  case 0x20:
    ExceptionList = &local_c;
    piVar2 = (int *)FUN_0059ed70();
    if (*piVar2 != 0) {
      puVar6 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x1c))();
    }
    break;
  case 0x7a:
    ExceptionList = &local_c;
    if (*(int *)this == 0) {
      ExceptionList = &local_c;
      iVar3 = FUN_005aaf90();
      iVar3 = *(int *)(iVar3 + 0x9ec);
      iVar4 = FUN_005aaf90();
      if (*(int *)(*(int *)(iVar4 + 8 + iVar3 * 4) + 8) != 1) break;
    }
    iVar3 = FUN_005aaf90();
    iVar3 = *(int *)(iVar3 + 0x9ec);
    pvVar5 = (void *)FUN_005aaf90();
    iVar3 = FUN_005a8a90(pvVar5,iVar3);
    if (iVar3 == 0) break;
    FUN_005c1020(0);
    FUN_005c0fa0();
    *(undefined4 *)this = 0;
    iVar3 = FUN_005aaf90();
    fVar7 = *(float *)(iVar3 + 0x9ec);
    pvVar5 = (void *)FUN_005aaf90();
    FUN_005ad530(pvVar5,fVar7);
    FUN_005c0e70();
    FUN_005c0e60();
    FUN_005c0e80();
    iVar3 = *(int *)((int)this + 0x184) + 1;
    *(int *)((int)this + 0x184) = iVar3;
    goto joined_r0x005a003e;
  case 0x7b:
    ExceptionList = &local_c;
    if (*(int *)this == 0) {
      ExceptionList = &local_c;
      iVar3 = FUN_005aaf90();
      iVar3 = *(int *)(iVar3 + 0x9ec);
      iVar4 = FUN_005aaf90();
      if (*(int *)(*(int *)(iVar4 + 8 + iVar3 * 4) + 8) != 1) break;
    }
    iVar3 = FUN_005aaf90();
    iVar3 = *(int *)(iVar3 + 0x9ec);
    iVar4 = FUN_005aaf90();
    if (*(int *)(*(int *)(iVar4 + 8 + iVar3 * 4) + 8) == 1) {
      iVar3 = FUN_005aaf90();
      iVar3 = *(int *)(iVar3 + 0x9ec);
      iVar4 = FUN_005aaf90();
      FUN_005c7e10(*(void **)(iVar4 + 8 + iVar3 * 4),0);
      *(undefined4 *)this = 1;
    }
    else {
      iVar3 = FUN_005aaf90();
      iVar3 = *(int *)(iVar3 + 0x9ec);
      iVar4 = FUN_005aaf90();
      FUN_005c7e10(*(void **)(iVar4 + 8 + iVar3 * 4),1);
      *(undefined4 *)this = 0;
    }
    break;
  case 0x7c:
    ExceptionList = &local_c;
    piVar2 = (int *)FUN_005aaf90();
    iVar3 = (**(code **)(*piVar2 + 0x1c))();
    if (iVar3 == 0) break;
    if (*(int *)this == 0) {
      iVar3 = FUN_005aaf90();
      iVar3 = *(int *)(iVar3 + 0x9ec);
      iVar4 = FUN_005aaf90();
      if (*(int *)(*(int *)(iVar4 + 8 + iVar3 * 4) + 8) != 1) break;
    }
    iVar3 = FUN_005aaf90();
    iVar3 = *(int *)(iVar3 + 0x9ec);
    iVar4 = FUN_005aaf90();
    if (*(int *)(*(int *)(iVar4 + 8 + (uint)(iVar3 == 0) * 4) + 0x84) != 0) break;
    piVar2 = (int *)FUN_005aaf90();
    (**(code **)(*piVar2 + 0x20))();
    iVar3 = *(int *)((int)this + 0x184) + 1;
    *(int *)((int)this + 0x184) = iVar3;
joined_r0x005a003e:
    if (iVar3 < 0) {
      *(undefined4 *)((int)this + 0x184) = 0;
    }
    break;
  case 0x7d:
    ExceptionList = &local_c;
    FUN_0059ef60((int)this);
  }
  FUN_0059f350(puVar6);
  ExceptionList = local_c;
  return;
}



int __thiscall FUN_0056a550(void *param_1,void *param_2)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  undefined4 *puVar4;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064b678;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_005081f0(param_1,param_2);
  if (iVar1 == 0) {
    ExceptionList = pvStack_c;
    return 0;
  }
  if (((byte)*(undefined4 *)((int)param_1 + 0x24) & 0xc0) == 0x40) {
    if (*(int *)((int)param_1 + 0x18) != 0x121) {
      FUN_0050a3c0(param_1,1,param_2);
      FUN_00509c30(param_1,100,param_2);
      pvVar3 = param_2;
      iVar2 = FUN_00559970();
      FUN_00509cc0(param_1,iVar2,pvVar3);
      iVar2 = *(int *)((int)param_1 + 0x5c);
      goto LAB_0056a6ba;
    }
LAB_0056a5d0:
    FUN_0050a3c0(param_1,1,param_2);
    FUN_00509c30(param_1,100,param_2);
    FUN_004fd450(&uStack_28,(int)param_2);
    puVar4 = &uStack_28;
    uStack_4 = 0;
    uStack_24 = 1;
    uStack_20 = 2;
    pvVar3 = (void *)FUN_00539fd0(param_1,2);
    FUN_004f9ca0(pvVar3,puVar4);
    puVar4 = &uStack_28;
    pvVar3 = (void *)FUN_00539fd0(param_1,2);
    FUN_0050f390(pvVar3,puVar4);
    puVar4 = &uStack_28;
    pvVar3 = (void *)FUN_00539fd0(param_1,2);
    FUN_0050fbf0(pvVar3,puVar4);
    puVar4 = &uStack_28;
    pvVar3 = (void *)FUN_00539fd0(param_1,2);
    FUN_0050fc80(pvVar3,puVar4);
    uStack_4 = 0xffffffff;
    FUN_004fd4d0(&uStack_28);
  }
  else {
    if (*(int *)((int)param_1 + 0x18) == 0x121) goto LAB_0056a5d0;
    pvVar3 = param_2;
    iVar2 = FUN_00559a10();
    FUN_0050a3c0(param_1,iVar2,pvVar3);
    if ((*(byte *)((int)param_1 + 0x88) & 1) != 0) {
      pvVar3 = param_2;
      FUN_005092f0((int)param_1);
      iVar2 = FUN_00559a20();
      FUN_00509c30(param_1,iVar2,pvVar3);
    }
  }
  pvVar3 = param_2;
  iVar2 = FUN_00559970();
  FUN_00509cc0(param_1,iVar2,pvVar3);
  iVar2 = *(int *)((int)param_1 + 0x5c);
LAB_0056a6ba:
  iVar2 = FUN_005599c0(iVar2);
  FUN_00509dc0(param_1,iVar2,param_2);
  ExceptionList = pvStack_c;
  return iVar1;
}


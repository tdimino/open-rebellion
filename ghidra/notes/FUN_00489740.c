
undefined4 * __thiscall FUN_00489740(void *this,undefined4 param_1,int param_2)

{
  undefined4 *puVar1;
  void *pvVar2;
  undefined4 uVar3;
  undefined4 *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634e3d;
  local_c = ExceptionList;
  this_00 = (undefined4 *)0x0;
  ExceptionList = &local_c;
  switch(param_1) {
  case 1:
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)thunk_FUN_005f5060((int)this + 0x34);
    if (puVar1 != (undefined4 *)0x0) {
      pvVar2 = (void *)FUN_00618b70(0xc4);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        this_00 = (undefined4 *)0x0;
      }
      else {
        this_00 = FUN_0049ac00(pvVar2,param_2,*(int *)this,(uint)puVar1);
      }
      local_4 = 0xffffffff;
      FUN_005f54d0((void *)((int)this + 0x34),puVar1[6]);
      (**(code **)*puVar1)(1);
    }
  default:
    goto switchD_0048976b_caseD_2;
  case 3:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 1;
    if (pvVar2 == (void *)0x0) {
LAB_00489f39:
      this_00 = (undefined4 *)0x0;
    }
    else {
      this_00 = FUN_004993e0(pvVar2,param_2,*(undefined4 *)this);
    }
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 3;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_004996f0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 5:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 5;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00499a30(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 6:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 4;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00499d50(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 7:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0xc;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00496080(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 9:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 2;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_004990b0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0xb:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 6;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00498c30(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0xc:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 7;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_004988f0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0xd:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 8;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00498140(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0xe:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 9;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00497890(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0xf:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0xa4);
    local_4 = 0xb;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00496460(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x10:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 10;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_004975e0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x13:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 0xd;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00495dd0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x14:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x7c);
    local_4 = 0xe;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00495460(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x15:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x70);
    local_4 = 0xf;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00494f80(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x16:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x98);
    local_4 = 0x10;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00492470(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x17:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x94);
    local_4 = 0x11;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00491260(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x18:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x14;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_00490ff0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x19:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x12;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_004902d0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x1a:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x13;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048fd80(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x1b:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x15;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048f990(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x1c:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x18;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048f160(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x1d:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x19;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048ed00(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x1e:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x16;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048f700(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x1f:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x17;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048e500(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x20:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x74);
    local_4 = 0x1a;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048e720(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x21:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x70);
    local_4 = 0x21;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048bd10(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x22:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x1e;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048b8e0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x23:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x1b;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048d3e0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x24:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x70);
    local_4 = 0x1c;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048d6f0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x25:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x1d;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048d070(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x26:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x1f;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048ce00(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x27:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(100);
    local_4 = 0x20;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048c940(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x28:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 0x22;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048c3a0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x29:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x23;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048b660(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x2b:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x24;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048afd0(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x2c:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x25;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048ab40(pvVar2,param_2,*(undefined4 *)this);
    break;
  case 0x2d:
    ExceptionList = &local_c;
    pvVar2 = (void *)FUN_00618b70(0x68);
    local_4 = 0x26;
    if (pvVar2 == (void *)0x0) goto LAB_00489f39;
    this_00 = FUN_0048adc0(pvVar2,param_2,*(undefined4 *)this);
  }
  local_4 = 0xffffffff;
switchD_0048976b_caseD_2:
  if (this_00 != (undefined4 *)0x0) {
    if (this_00[0x10] == 0) {
      (**(code **)*this_00)(1);
      this_00 = (undefined4 *)0x0;
    }
    else {
      uVar3 = FUN_0041d120();
      FUN_005f5600(this_00,uVar3);
    }
  }
  ExceptionList = local_c;
  return this_00;
}

